use alloc::format;

use crate::sync::OnceLock;
use crate::os::fs::Path;
use crate::os::env;

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