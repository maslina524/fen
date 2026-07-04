#[macro_export]
macro_rules! wide {
    ($item:expr) => {
        &$item.encode_utf16().chain(Some(0)).collect::<alloc::vec::Vec<u16>>()
    };
}