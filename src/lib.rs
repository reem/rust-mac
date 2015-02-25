#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # mac
//!
//! A collection of great and ubiqutitous macros.
//!

pub mod test;
pub mod mem;
pub mod format;
pub mod syntax_ext;
pub mod matches;

/// Unwraps an `Option` or returns from the function with the specified return
/// value.
///
/// Can be used on `Result`s by first calling `.ok()` or `.err()` on them.
///
/// # Examples
///
/// ```notest
/// # // FIXME: Import the macro here, currently the compiler complains about
/// # // missing `unwrap_or_return!`.
/// fn take_pair<I:Iterator>(iter: &mut I) -> Option<(<I as Iterator>::Item, <I as Iterator>::Item)> {
///    let first = unwrap_or_return!(iter.next(), None);
///    Some((first, unwrap_or_return!(iter.next(), None)))
/// }
/// ```
#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr, $r:expr) => (match $e { Some(e) => e, None => return $r, })
}
