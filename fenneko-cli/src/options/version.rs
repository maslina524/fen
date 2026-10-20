use core::convert::Infallible;

use common::env;

use crate::{actions::Action, render::Renderable};

#[allow(
    unused,
    reason = "These fields are not needed when rendering in the console, but will be useful for other rendering formats"
)]
pub struct VersionOut {
    name: &'static str,
    major: u8,
    minor: u8,
    build: u8,
    rev: u8,
    full: &'static str,
    os: &'static str,
}

impl Renderable for VersionOut {
    fn render_console(
        &self,
        f: &mut dyn core::fmt::Write,
    ) -> core::fmt::Result {
        let name = self.name;
        let full = self.full;
        let os = self.os;
        f.write_fmt(format_args!("{name} version {full}.{os}\n"))
    }
}

pub struct Version;

impl Action<VersionOut, Infallible> for Version {
    fn run(_args: Option<env::Args>) -> Result<VersionOut, Infallible> {
        fn version_part_as_u8(part: &'static str) -> u8 {
            part.parse::<u8>().unwrap_or_default()
        }
        let out = VersionOut {
            name: env!("CARGO_BIN_NAME"),
            major: version_part_as_u8(env!(
                "CARGO_PKG_VERSION_MAJOR"
            )),
            minor: version_part_as_u8(env!(
                "CARGO_PKG_VERSION_MINOR"
            )),
            build: version_part_as_u8(env!(
                "CARGO_PKG_VERSION_PATCH"
            )),
            rev: 0,
            full: env!("CARGO_PKG_VERSION"),
            os: env::consts::OS,
        };

        Ok(out)
    }
}
