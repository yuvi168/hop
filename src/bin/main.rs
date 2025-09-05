use std::fs;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let zsh_history_path = format!("/home/{}/.zsh_history",
        users::get_current_username()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let source: Vec<u8> = fs::read(zsh_history_path)?;
    let zsh_history = OsStr::from_bytes(&source[..])
        .to_string_lossy();
    let  regex = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
    for line in zsh_history.lines(){
        println!("{line}");
    }
    Ok(())
}

