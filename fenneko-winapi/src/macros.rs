#[macro_export]
macro_rules! handle {
    ($val:expr) => {{ $val as isize as $crate::raw::HANDLE }};
}

#[macro_export]
macro_rules! NT_SUCCESS {
    ($val:expr) => {{ $val as $crate::raw::NTSTATUS >= 0 }};
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
