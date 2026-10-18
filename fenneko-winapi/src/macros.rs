#[macro_export]
macro_rules! handle {
    ($val:expr) => {{ $val as isize as $crate::raw::HANDLE }};
}
