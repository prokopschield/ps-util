pub trait ResConv {
    fn ok<E>(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    fn err<T>(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> ResConv for T {}
