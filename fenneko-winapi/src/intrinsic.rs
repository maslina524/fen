use core::arch::asm;

/// # Safety
///
/// You must guarantee a valid offset.
#[must_use]
#[cfg(target_arch = "x86_64")]
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
#[cfg(target_arch = "x86")]
pub unsafe fn __readfsdword(offset: u32) -> u32 {
    let out: u32;
    // SAFETY: We do not guarantee safe operation of the function
    unsafe {
        asm!(
            "mov {}, fs:[eax]",
            out(reg) out,
            in("eax") offset,
            options(nostack, pure, readonly)
        );
    }
    out
}
