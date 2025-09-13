use crate::*;
use std::error::Error;

#[test]
fn check_path_retrieval() -> Result<(), Box<dyn Error>> {
    let path = get_path(String::from("lcode/lrust"))?;
    assert_eq!(path, String::from("lcode/lrust"));
    Ok(())
}

