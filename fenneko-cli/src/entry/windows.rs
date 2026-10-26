use core::arch::global_asm;

use winapi::win32::processthreadsapi::ExitProcess;

#[cfg(target_arch = "x86_64")]
global_asm!(
    ".global _start",
    "_start:",
    "    and rsp, -16",
    "    sub rsp, 32",
    "    call fenneko_entry_point",
    "    int 3",
);

#[cfg(target_arch = "x86")]
global_asm!(
    ".global _start",
    "_start:",
    "    and esp, -16",
    "    sub esp, 32",
    "    call fenneko_entry_point",
    "    int 3",
);

#[unsafe(no_mangle)]
pub extern "C" fn fenneko_entry_point() -> ! {
    let ret = crate::main();
    ExitProcess(ret)
}
