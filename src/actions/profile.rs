use alloc::vec::Vec;
use alloc::string::String;

use crate::{FenResult, println};
use crate::profile::get_profile;

pub fn profile(nn: &Vec<String>) -> FenResult<()> {
    if let Ok(p) = get_profile() {
        println!("{} <{}>", p.name, p.email);
    } else {
        println!("No profile");
    }
    Ok(())
}