// use advent_of_code::aoc::y2024::d02;

fn main() {
    let v = [0, 1, 2, 3, 4, 5];
    for i in 0..v.len() {
        let a = &v[..i];
        let b = &v[i + 1..];
        let mut c = a.to_vec();
        c.extend(b.iter());
        // dbg!(a);
        // dbg!(b);
        dbg!(c);
    }
    // dbg!(d02::part2());
}
