/// Convenience constructors for `Result` and `Option` values.
#[allow(clippy::missing_errors_doc)]
pub trait ToResult {
    /// Wraps `self` in `Ok(self)`.
    #[inline]
    fn ok<Err>(self) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(self)
    }

    /// Wraps `self` in `Err(self)`.
    #[inline]
    fn err<Any>(self) -> Result<Any, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    /// Wraps `self` in `Some(self)`.
    #[inline]
    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> ToResult for T {}

/// Conversions between `Result` and `Option` with optional value transformation.
pub trait ResConv<T, E> {
    /// Converts into `Option<T>`, dropping any error information.
    fn into_option(self) -> Option<T>;

    /// Converts into `Result<T, E>`.
    ///
    /// # Errors
    /// Returns `Err(E)` when the source value represents an absent or failed state.
    fn into_result(self) -> Result<T, E>;
}

impl<Input, Output, Err> ResConv<Output, Err> for Result<Input, Err>
where
    Input: Into<Output>,
{
    #[inline]
    fn into_option(self) -> Option<Output> {
        self.map(Into::into).ok()
    }

    #[inline]
    fn into_result(self) -> Result<Output, Err> {
        self.map(Into::into)
    }
}

impl<Input, Output, Err: Default> ResConv<Output, Err> for Option<Input>
where
    Input: Into<Output>,
{
    #[inline]
    fn into_option(self) -> Option<Output> {
        self.map(Into::into)
    }

    /// - Transforms `Some(T)` into `Ok(T)`.
    /// - Transforms `None` into `Err(Err::Default())`
    ///
    /// # Errors
    /// - Returns `Err(Err::Default())` if `self` is `None`.
    #[inline]
    fn into_result(self) -> Result<Output, Err> {
        self.map(Into::into).ok_or_else(Err::default)
    }
}
