use alloc::sync::Arc;

use crate::raw::env::{ArgsInner, args_string};

pub struct Args {
    inner: ArgsInner,
}

impl Iterator for Args {
    type Item = Arc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[must_use]
pub fn args() -> Args {
    let source = args_string();
    Args {
        inner: ArgsInner::new(source),
    }
}
