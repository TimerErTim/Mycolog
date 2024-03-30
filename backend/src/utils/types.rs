pub trait GenericTryInto {
    fn try_into_type<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }
}

impl<T: ?Sized> GenericTryInto for T {}
