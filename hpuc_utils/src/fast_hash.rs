use std::collections::{HashMap, HashSet};

// Todo -> Implement Fx hasher and use it in the fast map and fast set

/// A hash map optimized for fast hashing performance.
///
/// Currently an alias for `HashMap`, but will be replaced with a faster
/// hash implementation in the future.
pub type FastMap<K, V> = HashMap<K, V>;

/// A hash set optimized for fast hashing performance.
///
/// Currently an alias for `HashSet`, but will be replaced with a faster
/// hash implementation in the future.
pub type FastSet<K> = HashSet<K>;
