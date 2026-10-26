use crate::raw::kernel32;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum CodePage {
    Utf8 = 65001,
}

pub fn SetConsoleOutputCP(code_page: CodePage) {
    // SAFETY: wCodePage is valid, the return value is not checked, I
    // will check it when I create my own implementation, for now it
    // stays like this
    unsafe { kernel32::SetConsoleOutputCP(code_page as u32) };
}