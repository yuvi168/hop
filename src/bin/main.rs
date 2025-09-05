use std::fs;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
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
       hop::insert(path.to_string());
   }

    //finding the path with directory name
    let path_to_dir = hop::retrieve(String::from("somewhere/i/will/go/one/day"));
    println!("{path_to_dir}");
    Ok(())
}

