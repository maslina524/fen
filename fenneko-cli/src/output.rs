use alloc::boxed::Box;

use common::io::{Stderr, Stdout};

use crate::render::Renderable;

pub struct Output(Result<Box<dyn Renderable>, Box<dyn Renderable>>);

impl Output {
    pub fn new<O, E>(inner: Result<O, E>) -> Self
    where
        O: Renderable + 'static,
        E: Renderable + 'static,
    {
        Self(match inner {
            Ok(o) => Ok(Box::new(o)),
            Err(e) => Err(Box::new(e)),
        })
    }

    pub fn print(&self) -> i32 {
        match &self.0 {
            Ok(o) => {
                let mut stdout = Stdout::new();
                let _ = o.render_console(&mut stdout);
                0
            },
            Err(e) => {
                let mut stderr = Stderr::new();
                let _ = e.render_console(&mut stderr);
                1
            },
        }
    }
}
