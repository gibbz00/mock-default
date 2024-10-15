//! damock commons.

/// Mocking trait for composable test values.
///
/// Similar to [`std::default::Default`]
pub trait Mock: Sized {
    /// Returns `Self` containing a non-random and valid test value
    fn mock() -> Self;
}

impl<T: Mock> Mock for Option<T> {
    fn mock() -> Self {
        Some(Mock::mock())
    }
}

impl Mock for () {
    fn mock() -> Self {}
}

pub use damock_macros::Mock;
