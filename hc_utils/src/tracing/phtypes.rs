macro_rules! impl_phtype {
    ($name:ident, $val:expr) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name;

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str($val)
            }
        }
    };
}

impl_phtype!(PhB, "B");
impl_phtype!(PhE, "E");
impl_phtype!(PhX, "X");
impl_phtype!(Phi, "i");
impl_phtype!(PhM, "M");
impl_phtype!(PhC, "C");
