use core::arch::asm;

/// # Safety
///
/// You must guarantee a valid offset.
#[must_use]
pub unsafe fn __readgsqword(offset: u32) -> u64 {
    let out: u64;
    // SAFETY: We do not guarantee safe operation of the function
    unsafe {
        asm!(
            "mov {}, gs:[rax]",
            out(reg) out,
            in("rax") offset,
            options(nostack, pure, readonly)
        );
    }
    out
}

/// # Safety
///
/// You must guarantee a valid offset.
#[must_use]
pub unsafe fn __readfsqword(offset: u32) -> u64 {
    let out: u64;
    // SAFETY: We do not guarantee safe operation of the function
    unsafe {
        asm!(
            "mov {}, fs:[rax]",
            out(reg) out,
            in("rax") offset,
            options(nostack, pure, readonly)
        );
    }
    out
}
