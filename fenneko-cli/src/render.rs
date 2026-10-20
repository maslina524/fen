use core::{convert::Infallible, fmt};

pub trait Renderable {
    fn render_console(&self, f: &mut dyn fmt::Write) -> fmt::Result;
}

impl Renderable for Infallible {
    fn render_console(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }
}