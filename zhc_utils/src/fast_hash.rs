use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use rustc_hash::{FxBuildHasher, FxHasher};

/// A hash map optimized for fast hashing performance, using `FxHasher`.
pub type FastMap<K, V> = HashMap<K, V, FxBuildHasher>;

/// A hash set optimized for fast hashing performance, using `FxHasher`.
pub type FastSet<K> = HashSet<K, FxBuildHasher>;

/// Hashes a value using the same fast, non-cryptographic hasher as [`FastMap`]
/// and [`FastSet`].
pub fn fast_hash<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = FxHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}
