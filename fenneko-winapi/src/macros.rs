#[macro_export]
macro_rules! handle {
    ($val:expr) => {
        #[allow(trivial_numeric_casts, reason = "Cast isize to isize")]
        {
            $val as isize as $crate::raw::HANDLE
        }
    };
}

macro_rules! impl_tag {
    ($trait:ident : $($name:ident),+ $(,)?) => {
        $(
            impl $trait for $name {}
        )+
    };
}

macro_rules! impl_sealed {
    ($($name:ident),+ $(,)?) => {
        $(
            impl sealed::Sealed for $name {}
        )+
    };
}

macro_rules! mod_sealed {
    () => {
        mod sealed {
            pub trait Sealed {}
        }
    };
}
