use crate::raw::types as t;

/// Wine Source
/// <https://github.com/wine-mirror/wine/blob/master/include/winternl.h#L498>
#[repr(C)]
pub struct PEB {
    pub InheritedAddressSpace: t::BYTE,
    pub ReadImageFileExecOptions: t::BYTE,
    pub BeingDebugged: t::BYTE,
    pub BitField: t::BYTE,
    pub Mutant: t::PVOID,           // 0x08
    pub ImageBaseAddress: t::PVOID, // 0x10
    pub Ldr: t::PVOID,              // 0x18
    pub ProcessParameters: *const t::RTL_USER_PROCESS_PARAMETERS, /* 0x20 */
    pub SubSystemData: t::PVOID, // 0x28
    pub ProcessHeap: t::PVOID,   /* 0x30
                                  * etc. */
}
