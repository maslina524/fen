use common::env::Args;

use crate::render::Renderable;

pub mod help;
pub use help::Help;

pub trait Action<O: Renderable, E: Renderable> {
    fn run(args: Args) -> Result<O, E>;
}