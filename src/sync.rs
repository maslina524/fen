use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering, AtomicBool};

const INCOMPLETE: u8 = 0;
const INITIALIZING: u8 = 1;
const READY: u8 = 2;

pub struct OnceLock<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>> 
}

impl<T> OnceLock<T> {
    pub const fn new() -> Self {
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

unsafe impl<T: Sync> Sync for OnceLock<T> {}

pub struct Mutex {
    active: AtomicBool
}

impl Mutex {
    pub const fn new() -> Self {
        Self { active: AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        while self.active.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.active.store(false, Ordering::Release);
    } 
}

impl Drop for Mutex {
    fn drop(&mut self) {
        self.unlock();
    }
}