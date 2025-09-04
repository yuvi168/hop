use std::fs;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read("/home/yuvi/.zsh_history")?;
    let paths = OsStr::from_bytes(&source[..])
        .to_string_lossy();
    for line in paths.lines(){
        println!("{line}");
    }
    Ok(())
}

