use alloc::vec::Vec;
use alloc::string::String;

use crate::NoResult;
use crate::consts::git_config_path;
use crate::os::fs;
use crate::toml::Toml;

pub fn profile(nn: &Vec<String>) -> NoResult {
    crate::println!("{}", git_config_path());
    let toml_raw = fs::read_to_string(git_config_path())?;
    let toml = Toml::from_str(&toml_raw);
    crate::println!("{toml:#?}");

    Ok(())
}

fn get_git_profile() -> Option<(String, String)> {
    let toml_raw = fs::read_to_string(git_config_path()).ok()?;
    let toml = if let Some(t) = Toml::from_str(&toml_raw) { t } else {
        return None;
    };

    let user = if let Some(u) = toml.get("user") { u } else {
        return None;
    };

    let name = if let Some(n) = user.get("name") {
        if let Some(s) = n.as_str() { s } else {
            return None;
        }
    } else {
        return None;
    };

    let email = if let Some(n) = user.get("email") {
        if let Some(s) = n.as_str() { s } else {
            return None;
        }
    } else {
        return None;
    };

    Some( (name, email) )
} 