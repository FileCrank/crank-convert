// Generally useful macro_rules! macros that will be used across the project

#[macro_export]
/// Instantiate a HashSet like vec!
///
/// # Examples
/// ```rust
/// use std::collections::HashSet;
/// use crank_convert::set;
///
/// let x: HashSet<i32> = set![1, 2, 3];
/// assert!(x.contains(&2i32));
/// ```
macro_rules! set {
    ( $($n:expr ),* ) => {
        ::std::collections::HashSet::from([$($n, )*])
    }
}