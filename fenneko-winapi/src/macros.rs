#[macro_export]
macro_rules! handle {
    ($val:expr) => {{ $val as isize as $crate::raw::HANDLE }};
}

#[macro_export]
macro_rules! NT_SUCCESS {
    ($val:expr) => {{ $val as $crate::raw::NTSTATUS >= 0 }};
}

macro_rules! import {
    ($($name:ident),+ $(,)?) => {
        $(
            pub mod $name;
            pub use $name::$name;
        )+
    };
}
