use std::{collections::HashMap, hash::Hash};

pub fn invert<K, V: Eq + Hash>(hmap: HashMap<K, V>) -> HashMap<V, K> {
    hmap.into_iter().map(|(k, v)| (v, k)).collect()
}
