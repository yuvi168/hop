use std::error::Error;
use std::fs;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use regex::Regex;
use crate::query_database::insert_query;

pub fn retrieve_zsh_history() -> Result<(), Box<dyn Error>> {
    let zsh_history_path = format!(
        "/home/{}/.zsh_history",
        users::get_current_username()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let source: Vec<u8> = fs::read(zsh_history_path)?;
    let zsh_history = OsStr::from_bytes(&source[..])
        .to_string_lossy();
    let  re = Regex::new(r"cd\s*([^:\s]+)").unwrap();
    let mut paths: Vec<&str> = Vec::new();
    for (_, [path]) in re.captures_iter(&zsh_history).map(|c| c.extract()) {
        let path = path.trim();
        if !paths.contains(&path) {
            paths.push(&path);
        }
    }
    for path in paths {
        insert_query::insert(path.to_string());
    }
    Ok(())
}

