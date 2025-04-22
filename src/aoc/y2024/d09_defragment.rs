// Puzzle description: https://adventofcode.com/2024/day/9

use std::collections::{BTreeMap, BTreeSet};

fn is_file(sector_id: usize) -> bool {
    sector_id % 2 == 0
}

fn get_file_id(sector_id: usize) -> usize {
    assert!(is_file(sector_id));
    sector_id / 2
}

fn get_sector_len(c: u8) -> usize {
    assert!((c as char).is_ascii_digit());
    (c - b'0') as usize
}

// Checksum from file_id on block positions [begin, end)
fn partial_checksum(file_id: usize, begin: usize, len: usize) -> usize {
    let end = (begin + len).saturating_sub(1);
    let avg2 = begin + end;
    (len * avg2 * file_id) / 2
}

const INPUT: &str = "data/y2024/d09/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let sectors = input.as_bytes();
    let n_sec = sectors.len();
    let mut checksum = 0;

    // From disk start
    let mut head_sec_consumed = 0; // Nb of sectors consumed
    let mut head_blk_consumed = 0; // Blocks consumed within a sector
    let mut head_bx = 0; // Block index counted from disk start

    // From disk end
    // If last sector is free space, consider it already consumed
    let mut tail_sec_consumed = if 0 < n_sec && is_file(n_sec - 1) {
        0
    } else {
        1
    };
    let mut tail_blk_consumed = 0;

    while head_sec_consumed + tail_sec_consumed < n_sec {
        let head_sx = head_sec_consumed;
        let head_sec_len = get_sector_len(sectors[head_sx]);
        // From loop condition: head_sec_consumed < n_sec - tail_sec_consumed
        // Since head_sec_consumed >= 0, that means: 0 < n_sec - tail_sec_consumed
        // Meaning subracting 1 won't underflow.
        let tail_sx = n_sec - tail_sec_consumed - 1;
        let tail_sec_len = get_sector_len(sectors[tail_sx]);
        if is_file(head_sx) {
            // Consume all that remains of file at head
            let consumed_len = if head_sx == tail_sx {
                // File has been partially consumed from tail,
                head_sec_len - tail_blk_consumed
            } else {
                head_sec_len
            };
            checksum += partial_checksum(get_file_id(head_sx), head_bx, consumed_len);
            head_bx += consumed_len;
            head_sec_consumed += 1;
        } else {
            // Tail is always on a file sector
            // Data blocks not yet consumed from the tail sector
            let tail_sec_data_rem = tail_sec_len - tail_blk_consumed;
            // Free space remaining on head sector to write to
            let head_space = head_sec_len - head_blk_consumed;
            // Consume until we run out of space or data
            let consumed_len = tail_sec_data_rem.min(head_space);
            checksum += partial_checksum(get_file_id(tail_sx), head_bx, consumed_len);
            head_bx += consumed_len;
            tail_blk_consumed += consumed_len;
            head_blk_consumed += consumed_len;
            if tail_blk_consumed == tail_sec_len {
                // Advance tail to next file to consume (skip empty sector)
                tail_sec_consumed += 2;
                tail_blk_consumed = 0;
            }
            if head_blk_consumed == head_sec_len {
                // Advance head to next sector
                head_sec_consumed += 1;
                head_blk_consumed = 0;
            }
        }
    }
    checksum
}

struct File {
    id: usize,
    len: usize,
}

// Free segments sorted first by size, then by position
type SpaceMap = BTreeMap<usize, BTreeSet<usize>>;
// Key is file position
type FileMap = BTreeMap<usize, File>;

struct Fs {
    files: FileMap,
    // Adjacent free space is not merged, but that gives the right answer.
    space: SpaceMap,
}

// Find leftmost index of sufficient size to the left of given position
// Returns the block index where space was allocated
fn try_alloc(map: &mut SpaceMap, pos: usize, len: usize) -> Option<usize> {
    // (len, pos) of the found segment
    let mut found: Option<(usize, usize)> = None;
    for (new_len, new_poss) in map.iter().rev() {
        // Iterate over collections of free segments of sufficient size
        if *new_len < len {
            // Iterate in decreasing length, so all further lengths would be less.
            break;
        }
        if let Some(new_pos) = new_poss.first() {
            if let Some((_found_len, found_pos)) = found {
                // Update if we found a segment that is further left than previous candidate
                if *new_pos < found_pos {
                    found = Some((*new_len, *new_pos));
                }
            } else if *new_pos < pos {
                // Update if we found a segment that is left of given position
                found = Some((*new_len, *new_pos));
            }
        }
    }
    if let Some((found_len, found_pos)) = found {
        // Remove found segment
        map.get_mut(&found_len).unwrap().pop_first();
        // Insert shortened segment
        if len < found_len {
            let new_len = found_len - len;
            // We allocated from the start of the segment, so it is the start that moves
            let new_pos = found_pos + len;
            map.entry(new_len).or_default();
            map.get_mut(&new_len).unwrap().insert(new_pos);
        }
        Some(found_pos)
    } else {
        None
    }
}

impl Fs {
    fn new(sectors: &[u8]) -> Fs {
        let mut space: SpaceMap = BTreeMap::new();
        let mut files: FileMap = BTreeMap::new();
        let mut pos = 0;
        for (sector_id, sec) in sectors.iter().enumerate() {
            let len = get_sector_len(*sec);
            if is_file(sector_id) {
                files.insert(
                    pos,
                    File {
                        id: get_file_id(sector_id),
                        len,
                    },
                );
            } else {
                space.entry(len).or_default();
                space.get_mut(&len).unwrap().insert(pos);
            }
            pos += len;
        }
        Fs { space, files }
    }
    fn len(&self) -> usize {
        let file_end = self
            .files
            .last_key_value()
            .map(|(pos, file)| pos + file.len)
            .unwrap_or(0);
        let space_end = self
            .space
            .iter()
            .flat_map(|(len, poss)| poss.last().map(|pos| pos + len))
            .max();
        file_end.max(space_end.unwrap_or(0))
    }
    #[allow(unused)]
    fn show(&self) -> Vec<Option<usize>> {
        let mut ids = vec![None; self.len()];
        let mut written = vec![false; ids.len()];
        for (pos, file) in self.files.iter() {
            for i in (*pos..).take(file.len) {
                ids[i] = Some(file.id);
                assert!(!written[i]);
                written[i] = true;
            }
        }
        for (len, poss) in self.space.iter() {
            for pos in poss {
                for i in (*pos..).take(*len) {
                    ids[i] = None;
                    assert!(!written[i]);
                    written[i] = true;
                }
            }
        }
        // After defrag, not all will be written into,
        // because moved files do not yield their old space.
        // assert!(written.into_iter().all(|x| x));
        ids
    }
    #[allow(unused)]
    fn pretty_show(&self) -> String {
        let mut out = String::new();
        for s in self
            .show()
            .into_iter()
            .map(|id| id.map(|x| format!("{x}")).unwrap_or(".".to_string()))
        {
            out.push_str(s.as_str());
        }
        out
    }
    fn defrag(&mut self) {
        let mut new_files = BTreeMap::new();
        while let Some((old_pos, file)) = self.files.pop_last() {
            if let Some(new_pos) = try_alloc(&mut self.space, old_pos, file.len) {
                assert!(new_files.insert(new_pos, file).is_none());
            } else {
                assert!(new_files.insert(old_pos, file).is_none());
            }
        }
        self.files = new_files;
    }
    fn checksum(&self) -> usize {
        self.files
            .iter()
            .map(|(pos, file)| partial_checksum(file.id, *pos, file.len))
            .sum()
    }
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let sectors = input.trim().as_bytes(); // Trim EOF byte
    let mut fs = Fs::new(sectors);
    fs.defrag();
    fs.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6395800119709);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6418529470362);
    }
}
