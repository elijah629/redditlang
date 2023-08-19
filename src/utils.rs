use std::collections::HashSet;
use std::hash::Hash;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Checks if each item in an iterators is unique
#[inline]
pub fn is_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
