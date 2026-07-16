use alloc::string::String;

use crate::{NoResult, println};

const ESCAPE: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const YELLOW: &str = "\x1b[0;33m";

pub fn help(cmd: Option<String>) -> NoResult {
    println!("Git client fully written in Rust    {BOLD}(• ᴛ • マ!{ESCAPE}\n");

    println!("{YELLOW}Usage:{ESCAPE}");
    println!("  {BOLD}fen{ESCAPE} [-v | --version] [-p | --exec-path]");
    println!("      <command> [<args>]");

    println!("\n{YELLOW}Options:{ESCAPE}");
    option("-v", "--version", "Prints the version");
    option("-p", "--exec-path", "Prints the path to the executable file");

    println!("\n{YELLOW}Actions:{ESCAPE}");
    action("init", "Initializes a new Git repository");
    action("add", "Adds files to the staging area");
    action("commit", "Creates a commit of the files in the staging area");
    action("help", "Show this message");
    
    println!();

    Ok(())
}

#[inline(always)]
fn option(small: &str, long: &str, desc: &str) {
    if small.is_empty() {
        println!("  \t{BOLD}{long}{ESCAPE}\t{desc}");
    } else {
        println!("  {BOLD}{small}{ESCAPE},\t{BOLD}{long}{ESCAPE}\t{desc}");
    }
}

#[inline(always)]
fn action(cmd: &str, desc: &str) {
    let len = cmd.len();
    let mut offset = if len < 8 {
        8 - len
    } else {
        0
    };
    println!("  {BOLD}{cmd}{}{ESCAPE}\t{desc}", " ".repeat(offset));
}