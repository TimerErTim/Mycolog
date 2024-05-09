pub trait GenericTryInto {
    fn try_into_type<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }
}

impl<T: ?Sized> GenericTryInto for T {}

pub trait AnyhowExt<T, E: Into<anyhow::Error>> {
    fn anyhow(self) -> anyhow::Result<T>;
}

impl<T, E: Into<anyhow::Error>> AnyhowExt<T, E> for Result<T, E> {
    fn anyhow(self) -> anyhow::Result<T> {
        self.map_err(|err| err.into())
    }
}
