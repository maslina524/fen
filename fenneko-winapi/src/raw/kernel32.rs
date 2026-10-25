use crate::{link, raw::types as t};

// TODO: Replace with own implementation using only ntdll
link!("kernel32" fn SetConsoleOutputCP(wCodePageID: t::UINT) -> t::BOOL);
