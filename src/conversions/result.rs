pub trait ToResult {
    #[inline(always)]
    fn ok<Err>(self) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(self)
    }

    #[inline(always)]
    fn err<Any>(self) -> Result<Any, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    #[inline(always)]
    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> ToResult for T {}

pub trait ResConv<T, E: Default> {
    fn to_option(self) -> Option<T>;
    fn to_result(self) -> Result<T, E>;
}

impl<Input, Output, Err: Default> ResConv<Output, Err> for Result<Input, Err>
where
    Input: Into<Output>,
{
    #[inline(always)]
    fn to_option(self) -> Option<Output> {
        match self {
            Ok(value) => value.into().some(),
            _ => None,
        }
    }

    #[inline(always)]
    fn to_result(self) -> Result<Output, Err> {
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
    #[inline(always)]
    fn to_option(self) -> Option<Output> {
        match self {
            Some(value) => value.into().some(),
            _ => None,
        }
    }

    #[inline(always)]
    fn to_result(self) -> Result<Output, Err> {
        match self {
            Some(value) => value.into().ok(),
            _ => Err::default().err(),
        }
    }
}
