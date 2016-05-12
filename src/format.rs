//! Macros for string formatting.

/// Conditionally perform string formatting.
///
/// If `$enabled` is true, then do the formatting and return a `Cow::Owned`.
///
/// Otherwise, just return the borrowed (often `'static`) string
/// `$borrowed`.
///
/// When `$enabled` is false, this avoids the overhead of allocating
/// and writing to a buffer, as well as any overhead or side effects
/// of the format arguments.
///
/// # Example
///
/// You can use `format_if` to implement a detailed error logging facility
/// that can be enabled at runtime.
///
/// ```
/// # #[macro_use] extern crate mac;
/// # use std::borrow::Cow;
/// # fn main() {
/// let formatted = format_if!(true, "Vague error", "Error code {:?}", 3);
///
/// assert_eq!(&formatted[..], "Error code 3");
/// match formatted {
///     Cow::Owned(..) => {},
///     _ => panic!("not owned"),
/// }
///
/// let not_formatted = format_if!(false, "Vague error", "Error code {:?}", {
///     // Note that the argument is not evaluated.
///     panic!("oops");
/// });
///
/// assert_eq!(&not_formatted[..], "Vague error");
/// match not_formatted {
///     Cow::Borrowed(..) => {},
///     _ => panic!("not borrowed"),
/// }
/// # }
/// ```
#[macro_export]
macro_rules! format_if {
    ($enabled:expr, $borrowed:expr, $fmt:expr, $($args:expr),*) => {
        if $enabled {
            ::std::borrow::Cow::Owned(format!($fmt, $($args),*))
        } else {
            ::std::borrow::Cow::Borrowed($borrowed)
        }
    }
}
