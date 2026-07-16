use alloc::collections::BTreeMap;
use alloc::format;

use crate::sync::OnceLock;
use crate::os::fs::Path;
use crate::os::env;

pub const FILE_ICON: &str = "\u{ea7b}";

static USER_PATH: OnceLock<Path> = OnceLock::new();

pub fn user_path() -> &'static Path {
    USER_PATH.get_or_init(|| {
        let user = env::get_user_name();
        Path::from_str(&format!("C:/Users/{user}"))
    })
}

static GIT_CONFIG_PATH: OnceLock<Path> = OnceLock::new();

pub fn git_config_path() -> &'static Path {
    GIT_CONFIG_PATH.get_or_init(|| {
        user_path().clone().join(".gitconfig")
    })
}

static EXTENSION_COLORED_ICONS: OnceLock<BTreeMap<&str, &str>> = OnceLock::new();

pub fn extension_colored_icons<'a>() -> &'static BTreeMap<&'a str, &'a str> {
    EXTENSION_COLORED_ICONS.get_or_init(|| {
        let mut ret = BTreeMap::new();

        ret.insert("rs", "\x1b[0;38;2;109;127;134;49m\u{e7a8}"); // Rust
        ret.insert("c", "\x1b[0;38;2;101;154;210;49m\u{e61e}"); // C
        ret.insert("c", "\x1b[0;38;2;81;154;181;49m\u{e606}"); // Python

        ret
    })
}