use core::{convert::Infallible, fmt};

use common::prelude::*;
use proc::ansi;

use crate::{actions::Action, render::Renderable};

static HELP_OPTIONS: &[HelpOutOption] = &[HelpOutOption::new(
    Some("-v"),
    Some("--version"),
    "Prints the version",
)];

static HELP_ACTIONS: &[HelpOutAction] =
    &[HelpOutAction::new("help", "Show this message")];

pub struct HelpOutOption {
    small: Option<&'static str>,
    long: Option<&'static str>,
    desc: &'static str,
}

impl HelpOutOption {
    pub const fn new(
        small: Option<&'static str>,
        long: Option<&'static str>,
        desc: &'static str,
    ) -> Self {
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
    actions: &'static [HelpOutAction],
}

impl HelpOut {
    pub const fn new(
        head: &'static str,
        options: &'static [HelpOutOption],
        actions: &'static [HelpOutAction],
    ) -> Self {
        Self {
            head,
            options,
            actions,
        }
    }
}

impl Renderable for HelpOut {
    fn render_console(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        fn render_option_in_usage(
            f: &mut dyn fmt::Write,
            option: &HelpOutOption,
        ) -> fmt::Result {
            match (option.small, option.long) {
                (Some(s), Some(l)) => {
                    f.write_fmt(format_args!(" [{s} | {l}]"))
                },
                (None, Some(l)) => {
                    f.write_fmt(format_args!(" [{l}]"))
                },
                (Some(s), None) => {
                    f.write_fmt(format_args!(" [{s}]"))
                },
                (None, None) => Ok(()),
            }
        }

        fn render_option(
            f: &mut dyn fmt::Write,
            option: &HelpOutOption,
        ) -> fmt::Result {
            match (option.small, option.long) {
                (Some(s), Some(l)) => {
                    f.write_fmt(format_args!("  {s},\t{l:<10}  \t"))?;
                },
                (None, Some(l)) => {
                    f.write_fmt(format_args!(" \t{l:<10}  \t"))?;
                },
                (Some(s), None) => {
                    f.write_fmt(format_args!(" {s}\t{:<10} \t", ""))?;
                },
                (None, None) => {},
            }
            f.write_str(option.desc)
        }

        fn render_action(
            f: &mut dyn fmt::Write,
            action: &HelpOutAction,
        ) -> fmt::Result {
            let name = action.name;
            let desc = action.desc;
            f.write_fmt(format_args!("  {name:<10}{desc}"))
        }

        // Head
        f.write_str(self.head)?;

        // Usage
        f.write_str(ansi!("\n\n[yellow][bold]Usage:[esc]\n"))?;
        f.write_str("  fen")?;
        for o in self.options {
            render_option_in_usage(f, o)?;
        }
        f.write_str("\n      <command> [<args>]")?;

        // Options
        f.write_str(ansi!("\n\n[yellow][bold]Options:[esc]\n"))?;
        for o in self.options {
            render_option(f, o)?;
        }

        // Actions
        f.write_str(ansi!("\n\n[yellow][bold]Actions:[esc]\n"))?;
        for a in self.actions {
            render_action(f, a)?;
        }

        Ok(())
    }
}

pub struct Help;

impl Action<HelpOut, Infallible> for Help {
    fn run(_args: Option<env::Args>) -> Result<HelpOut, Infallible> {
        let out = HelpOut::new(
            "Git client written entirely in Rust   (• ᴛ • マ!",
            HELP_OPTIONS,
            HELP_ACTIONS,
        );

        Ok(out)
    }
}
