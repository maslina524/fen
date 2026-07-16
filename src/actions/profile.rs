use alloc::vec::Vec;
use alloc::string::String;

use crate::{NoResult, println};
use crate::profile::get_profile;

pub fn profile(nn: &Vec<String>) -> NoResult {
    if let Some(p) = get_profile() {
        println!("{} <{}>", p.name, p.email);
    } else {
        println!("No profile");
    }
    Ok(())
}