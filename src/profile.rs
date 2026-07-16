use alloc::string::String;

use crate::{FenResult, toml::Toml};

pub struct Profile {
    pub name: String,
    pub email: String
}

fn get_profile_inner() -> Option<Profile> {
    let toml = Toml::open_git_config()?;
    let user = toml.get("user")?;
    let name = user.get("name")?.as_str()?;
    let email = user.get("email")?.as_str()?;
    Some( Profile { name, email } )
}

pub fn get_profile() -> FenResult<Profile> {
    get_profile_inner().ok_or("profile not found, use fen `profile --name [name] --email [email]` to create a new profile.".into())
}