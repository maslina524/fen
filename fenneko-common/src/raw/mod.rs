#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::*;
