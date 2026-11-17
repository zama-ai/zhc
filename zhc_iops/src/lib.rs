use zhc_builder::Builder;

macro_rules! builder_ext {
    ($(fn $method:ident = $func:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
        pub trait BuilderExt {
            $(fn $method(&self, $($arg: $ty),*) -> $ret;)*
        }

        impl BuilderExt for Builder {
            $(fn $method(&self, $($arg: $ty),*) -> $ret {
                ::zhc_iops_r2l::$func(self, $($arg),*)
            })*
        }
    };
}

::zhc_iops_r2l::for_each_export!(builder_ext);
