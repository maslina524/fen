#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused)]

pub type NoResult = Result<(), Box<dyn core::error::Error>>;

use alloc::collections::*;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use alloc::format;

use crate::os::io;
use crate::os::env;
use crate::args::ArgsParser;

mod os;
mod actions;
mod zlib;

mod toml;
mod sha1;
mod glob;
mod sync;
mod args;
mod blob;
mod indx;
mod tree;
mod commit;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: os::noalloc::WinAllocator = os::noalloc::WinAllocator;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;
    use crate::{os::windows::ExitProcess, println};

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("{}", info.message());
        
        if let Some(loc) = info.location() {
            println!("{}:{}:{}", loc.file(), loc.line(), loc.column());
        }
        
        unsafe { ExitProcess(101) }
    }
}

fn version() -> NoResult {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    println!("Fen v{version}");
    Ok(())
}

fn exec_path() -> NoResult {
    let path = os::env::current_exe();
    println!("{path}");
    Ok(())
}

/// If this cfg is used rust-analyzer will not process this function
/// and if the cfg is commented out running cargo test will cause an
/// entry point err idk how to fix this so the attr is commented out
/// when you need to run the test you have to uncomment the attr
#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    io::set_console_to_utf8();

    let args = env::args();

    let mut parser = ArgsParser::new("fen", "git client in rust");
    parser.add_arg("message", Some('m'), true);

    parser.add_arg("version", Some('v'), false);
    parser.add_arg("exec-path", Some('p'), false);

    let parsed = parser.parse(&args[1..]);

    if let Err(err) = handler(parser) {
        eprintln!("Fen: {err}");
        return 1;
    }

    return 0;
}

fn handler(parser: ArgsParser) -> NoResult {
    let args = &env::args()[1..];
    let parsed = parser.parse(args)?;

    match parsed.action {
        Some(sub) => {
            return match sub.as_str() {
                "init"   => actions::init(),
                "add"    => actions::add(&parsed.nn[..]),
                "commit" => actions::commit(parsed.map.get("message")),
                _ => Err(format!("`{sub}` not a fen command").into())
            };
        },
        None => {
            return if parsed.map.contains_key("version") {
                version()
            } else if parsed.map.contains_key("exec-path") {
                exec_path()
            } else {
                Err("help page".into())
            };
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::io;
    use crate::os::fs;
    use crate::zlib;
    
    extern crate std;

    #[test]
    fn zlib_decompress() {
        io::set_console_to_utf8();

        let string = std::fs::read(".git/objects/32/d7e129e6685b5bb82844f81bf4fd692d0091be").unwrap();
        let mut decoded = Vec::new();
        zlib::decompress(string, &mut decoded);

        let mut ret = String::new();
        for byte in &decoded {
            if (0x20..=0x7E).contains(byte) {
                ret.push(*byte as char);
            } else {
                ret.push_str(&format!(r"\x{byte:02x}"));
            }
        }
        crate::println!("Data: \nb\"{ret}\"\n\nRaw: \n{decoded:?}");
    }

    #[test]
    fn zlib_compress() {
        let string: Vec<u8> = "Hello World!".bytes().collect();

        let mut encoded = Vec::new();
        zlib::compress(&string, &mut encoded);

        let mut decoded = Vec::new();
        zlib::decompress(encoded, &mut decoded);
        
        assert_eq!(string, decoded)
    }

    #[test]
    fn read_file() {
        let result = fs::read_to_bytes("src/main.rs");
        assert!(result.is_ok());

        let content = String::from_utf8(result.unwrap()).unwrap();
        println!("{content}")
    }
}