use core::ffi::{c_int, c_void};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let d = dest.cast::<u8>();
    let s = src.cast::<u8>();
    for i in 0..n {
        // SAFETY: Moving the pointer within the buffer
        unsafe { *d.add(i) = *s.add(i) };
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let d = dest.cast::<u8>();
    let s = src.cast::<u8>();
    if (d as usize) < (s as usize) {
        for i in 0..n {
            // SAFETY: Moving the pointer within the buffer
            unsafe {
                *d.add(i) = *s.add(i);
            }
        }
    } else {
        for i in (0..n).rev() {
            // SAFETY: Moving the pointer within the buffer
            unsafe {
                *d.add(i) = *s.add(i);
            }
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    let d = dest.cast::<u8>();
    for i in 0..n {
        // SAFETY: Moving the pointer within the buffer
        unsafe { *d.add(i) = c as u8 };
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int {
    let a = a.cast::<u8>();
    let b = b.cast::<u8>();
    for i in 0..n {
        // SAFETY: Moving the pointer within the buffer
        unsafe {
            let diff = i32::from(*a.add(i)) - i32::from(*b.add(i));
            if diff != 0 {
                return diff;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int {
    // SAFETY: Just C fn
    unsafe { memcmp(a, b, n) }
}

#[unsafe(no_mangle)]
pub const unsafe extern "C" fn strlen(s: *const core::ffi::c_char) -> usize {
    let s = s.cast::<u8>();
    let mut len = 0;
    // SAFETY: Moving the pointer within the buffer
    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }
    len
}
