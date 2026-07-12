use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};

const INCOMPLETE: u8 = 0;
const INITIALIZING: u8 = 1;
const READY: u8 = 2;

pub struct OnceLock<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>> 
}

impl<T> OnceLock<T> {
    pub fn new() -> Self {
        Self { state: AtomicU8::new(INCOMPLETE), value: UnsafeCell::new(MaybeUninit::uninit()) }
    }

    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        if self.state.load(Ordering::Acquire) == READY {
            unsafe { return (&*self.value.get()).assume_init_ref() };
        }

        match self.state.compare_exchange(
            INCOMPLETE,
            INITIALIZING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                let value = f();
                unsafe {
                    (*self.value.get()).write(value);
                }
                self.state.store(READY, Ordering::Release);
                unsafe { (&*self.value.get()).assume_init_ref() }
            }
            Err(_) => {
                while self.state.load(Ordering::Acquire) != READY {
                    core::hint::spin_loop();
                }
                unsafe { (&*self.value.get()).assume_init_ref() }
            }
        }
    }
}