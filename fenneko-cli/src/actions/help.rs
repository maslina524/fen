use core::convert::Infallible;

use common::prelude::*;
use proc::ansi;

use crate::{actions::Action, render::Renderable};

static HELP: &str = proc::ansi!(
"

[yellow][bold]Usage:[esc]
  fen [-v | --version]
      <command> [<args>]

[yellow][bold]Options:[esc]
  -v,\t--version       Prints the version

[yellow][bold]Actions:[esc]
  help          Show this message
"
);

static HELP_OPTIONS: &[HelpOutOption] = &[
    HelpOutOption::new(Some("-v"), Some("--version"), "Prints the version")
];

static HELP_ACTIONS: &[HelpOutAction] = &[
    HelpOutAction::new("help", "Show this message")
];

pub struct HelpOutOption {
    small: Option<&'static str>,
    long: Option<&'static str>,
    desc: &'static str,
}

impl HelpOutOption {
    pub const fn new(small: Option<&'static str>, long: Option<&'static str>, desc: &'static str) -> Self {
        Self { small, long, desc }
    }
}

pub struct HelpOutAction {
    name: &'static str,
    desc: &'static str,
}

impl HelpOutAction {
    pub const fn new(name: &'static str, desc: &'static str) -> Self {
        Self { name, desc }
    }
}

pub struct HelpOut {
    head: &'static str,
    options: &'static [HelpOutOption],
    actions: &'static [HelpOutAction]
}

impl HelpOut {
    pub const fn new(head: &'static str, options: &'static [HelpOutOption], actions: &'static [HelpOutAction]) -> Self {
        Self { head, options, actions }
    }
}

impl Renderable for HelpOut {
    fn render_console(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.head)?;

        f.write_str(ansi!("\n\n[yellow][bold]Usage:[esc]\n"))?;

        Ok(())
    }
}

pub struct Help;

impl Action<HelpOut, Infallible> for Help {
    fn run(_args: env::Args) -> Result<HelpOut, Infallible> {
        let out = HelpOut::new(
            "Git client written entirely in Rust   (• ᴛ • マ!", 
            HELP_OPTIONS, 
            HELP_ACTIONS
        );

        Ok(out)
    }
}