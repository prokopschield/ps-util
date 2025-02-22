#[allow(clippy::missing_errors_doc)]
pub trait ToResult {
    #[inline]
    fn ok<Err>(self) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(self)
    }

    #[inline]
    fn err<Any>(self) -> Result<Any, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    #[inline]
    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> ToResult for T {}

#[allow(clippy::missing_errors_doc)]
pub trait ResConv<T, E> {
    fn into_option(self) -> Option<T>;
    fn into_result(self) -> Result<T, E>;
}

impl<Input, Output, Err> ResConv<Output, Err> for Result<Input, Err>
where
    Input: Into<Output>,
{
    #[inline]
    fn into_option(self) -> Option<Output> {
        self.map_or_else(|_| None, |value| value.into().some())
    }

    #[inline]
    fn into_result(self) -> Result<Output, Err> {
        match self {
            Ok(value) => value.into().ok(),
            Err(err) => err.err(),
        }
    }
}

impl<Input, Output, Err: Default> ResConv<Output, Err> for Option<Input>
where
    Input: Into<Output>,
{
    #[inline]
    fn into_option(self) -> Option<Output> {
        self.map_or_else(|| None, |value| value.into().some())
    }

    /// - Transforms `Some(T)` into `Ok(T)`.
    /// - Transforms `None` into `Err(Err::Default())`
    ///
    /// # Errors
    /// - Returns `Err(Err::Default())` if `self` is `None`.
    #[inline]
    fn into_result(self) -> Result<Output, Err> {
        self.map_or_else(|| Err::default().err(), |value| value.into().ok())
    }
}
