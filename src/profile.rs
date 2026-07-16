use alloc::string::String;

use crate::toml::Toml;

pub struct Profile {
    pub name: String,
    pub email: String
}

pub fn get_profile() -> Option<Profile> {
    let toml = Toml::open_git_config()?;
    let user = toml.get("user")?;
    let name = user.get("name")?.as_str()?;
    let email = user.get("email")?.as_str()?;
    Some( Profile { name, email } )
}