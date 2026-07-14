use alloc::string::String;

use crate::{commit, indx, tree};
use crate::{NoResult, println};

pub fn commit(msg: Option<&String>) -> NoResult {
    let msg = if let Some(msg) = msg { msg } else {
        return Err("the commit requires a message, specify it using the -m or --message flag".into());
    };

    let index = indx::read_index()?;
    let tree_hash = tree::write_tree(&index, "")?;

    commit::write_commit(&tree_hash, msg)?;

    Ok(())
}