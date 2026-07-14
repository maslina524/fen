use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use crate::os::error::ErrorCode;
use crate::os::windows::*;

const TIME_ZONE_ID_DAYLIGHT: u32 = 2;
const TIME_ZONE_ID_INVALID: u32 = 0xFFFFFFFF;

pub fn args() -> Vec<String> {
    let ptr = unsafe { GetCommandLineW() };
    
    let mut argv_count = 0;
    let mut raw_argv_ptrs = unsafe { CommandLineToArgvW(
        ptr, 
        &mut argv_count as *mut i32
    ) };

    let mut argv_ptrs = Vec::new();
    for _ in 0..argv_count {
        let arg_ptr = unsafe { *raw_argv_ptrs };
        argv_ptrs.push(arg_ptr);
        raw_argv_ptrs = unsafe { raw_argv_ptrs.add(1) };
    }

    let mut ret_argv = Vec::new();
    for arg_ptr in argv_ptrs {
        let mut len = 0;
        while unsafe { *arg_ptr.add(len) } != 0 {
            len += 1;
        }
        let slice = unsafe { core::slice::from_raw_parts(arg_ptr, len as usize) };
        let string = String::from_utf16_lossy(slice);
        ret_argv.push(string);
    }

    ret_argv
}

pub fn current_exe() -> String {
    let mut buf = [0u16; 1024];
    let len = unsafe { GetModuleFileNameW(core::ptr::null_mut(), &mut buf as *mut u16, 1024) };

    let slice = unsafe { core::slice::from_raw_parts(&buf as *const u16, len as usize) };
    let string = String::from_utf16_lossy(slice);

    string
}

pub fn timestamp() -> u64 {
    let mut time = unsafe { core::mem::zeroed() };
    unsafe { GetSystemTimeAsFileTime(&mut time) };

    let u64_time = ((time.dwHighDateTime as u64) << 32) | time.dwLowDateTime as u64;
    let timestamp = (u64_time - 116_444_736_000_000_000) / 10_000_000;
    timestamp
}

pub fn get_time_zone() -> i16 {
    let mut tzi = unsafe { core::mem::zeroed() };
    let ret = unsafe { GetTimeZoneInformation(&mut tzi) };
    if ret == TIME_ZONE_ID_INVALID {
        ErrorCode::last().panic()
    }

    let mut bias_mins = tzi.Bias;

    if ret == TIME_ZONE_ID_DAYLIGHT {
        bias_mins += tzi.DaylightBias;
    }

    (-bias_mins) as i16
}

pub fn get_time_zone_string() -> String {
    let mut tz = get_time_zone();
    let symb = if tz < 0 { '-' } else { '+' };
    tz = tz.abs();
    let hours = tz / 60;
    let mins = tz % 60;

    format!("{symb}{hours:02}{mins:02}")
}

#[cfg(test)]
mod tests {
    use crate::os::io;
    use crate::os::env;
    
    extern crate std;

    #[test]
    fn time_zone_string() {
        io::set_console_to_utf8();
        let string = env::get_time_zone_string();
        crate::println!("{string}");
    }
}