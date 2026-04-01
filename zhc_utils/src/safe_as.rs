pub trait SafeAs {
    fn sas<T>(self) -> T
    where
        Self: TryInto<T>;
}

impl<A> SafeAs for A {
    #[track_caller]
    fn sas<T>(self) -> T
    where
        Self: TryInto<T>,
    {
        match self.try_into() {
            Ok(v) => v,
            Err(_) => {
                let loc = std::panic::Location::caller();
                panic!("Failed to safe cast at {}:{}", loc.file(), loc.line());
            }
        }
    }
}
