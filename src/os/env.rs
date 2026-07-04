use alloc::string::String;
use alloc::vec::Vec;

use crate::os::windows::*;

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