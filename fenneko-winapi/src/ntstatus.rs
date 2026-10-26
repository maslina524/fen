use core::fmt;

use crate::raw::NTSTATUS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Success,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Customer {
    Microsoft,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Facility {
    Null,
    Debugger,
    RpcRuntime,
    RpcStubs,
    IoErrorCode,
    NtWin32,
    NtSspi,
    TerminalServer,
    MuiErrorCode,
    UsbErrorCode,
    HidErrorCode,
    FirewireErrorCode,
    ClusterErrorCode,
    AcpiErrorCode,
    SxsErrorCode,
    Transaction,
    CommonLog,
    Video,
    FilterManager,
    Monitor,
    GraphicsKernel,
    DriverFramework,
    FveErrorCode,
    FwpErrorCode,
    NdisErrorCode,
    Hypervisor,
    Ipsec,
    MaximumValue,
    Unknown(u16),
}

impl Facility {
    #[must_use]
    pub const fn from_raw(value: u16) -> Self {
        match value {
            0x000 => Self::Null,
            0x001 => Self::Debugger,
            0x002 => Self::RpcRuntime,
            0x003 => Self::RpcStubs,
            0x004 => Self::IoErrorCode,
            0x007 => Self::NtWin32,
            0x009 => Self::NtSspi,
            0x00A => Self::TerminalServer,
            0x00B => Self::MuiErrorCode,
            0x010 => Self::UsbErrorCode,
            0x011 => Self::HidErrorCode,
            0x012 => Self::FirewireErrorCode,
            0x013 => Self::ClusterErrorCode,
            0x014 => Self::AcpiErrorCode,
            0x015 => Self::SxsErrorCode,
            0x019 => Self::Transaction,
            0x01A => Self::CommonLog,
            0x01B => Self::Video,
            0x01C => Self::FilterManager,
            0x01D => Self::Monitor,
            0x01E => Self::GraphicsKernel,
            0x020 => Self::DriverFramework,
            0x021 => Self::FveErrorCode,
            0x022 => Self::FwpErrorCode,
            0x023 => Self::NdisErrorCode,
            0x035 => Self::Hypervisor,
            0x036 => Self::Ipsec,
            0x037 => Self::MaximumValue,
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Ntstatus(u32);

impl Ntstatus {
    #[must_use]
    pub const fn new(status: NTSTATUS) -> Self {
        Self(status.cast_unsigned())
    }

    #[must_use]
    pub const fn is_success(self) -> bool {
        self.0.cast_signed() >= 0
    }

    #[must_use]
    pub const fn into_inner(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn severity(self) -> Severity {
        let val = self.0 >> 30;
        match val {
            0b00 => Severity::Success,
            0b01 => Severity::Info,
            0b10 => Severity::Warn,
            0b11 => Severity::Error,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn code(self) -> u16 {
        (self.0 & 0x0000_FFFF) as u16
    }

    #[must_use]
    pub const fn facility(self) -> Facility {
        let val = (self.0 >> 16) & 0x0FFF;
        Facility::from_raw(val as u16)
    }

    #[must_use]
    pub const fn customer(self) -> Customer {
        if (self.0 >> 29) & 1 == 1 {
            Customer::Microsoft
        } else {
            Customer::Other
        }
    }

    #[must_use]
    pub const fn is_microsoft(self) -> bool {
        matches!(self.customer(), Customer::Microsoft)
    }
}

impl fmt::Display for Ntstatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.into_inner())
    }
}

impl fmt::Debug for Ntstatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ntstatus")
            .field("code", &self.code())
            .field("facility", &self.facility())
            .field("customer", &self.customer())
            .field("severity", &self.severity())
            .finish()
    }
}

impl From<NTSTATUS> for Ntstatus {
    fn from(value: NTSTATUS) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUCCESS: Ntstatus = Ntstatus::new(0);
    const TIMEOUT: Ntstatus = Ntstatus::new(0x0102);
    const OBJECT_NAME_EXISTS: Ntstatus = Ntstatus::new(0x4000_0000);
    const BUFFER_OVERFLOW: Ntstatus = Ntstatus::new(0x8000_0005u32.cast_signed());
    const INVALID_HANDLE: Ntstatus = Ntstatus::new(0xC000_0008u32.cast_signed());

    #[test]
    fn is_success_test() {
        assert!(SUCCESS.is_success());
        assert!(TIMEOUT.is_success());
    }

    #[test]
    fn severity_test() {
        assert_eq!(TIMEOUT.severity(), Severity::Success);
        assert_eq!(OBJECT_NAME_EXISTS.severity(), Severity::Info);
        assert_eq!(BUFFER_OVERFLOW.severity(), Severity::Warn);
        assert_eq!(INVALID_HANDLE.severity(), Severity::Error);
    }

    #[test]
    fn code_test() {
        assert_eq!(TIMEOUT.code(), 0x0102);
        assert_eq!(INVALID_HANDLE.code(), 0x08);
    }

    #[test]
    fn debug_print_test() {
        println!("{INVALID_HANDLE:?}");
    }
}
