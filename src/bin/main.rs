use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //hop::retrieve_zsh_history()?;
    let path_to_dir = hop::get_path(String::from("lrust"))?;
    println!("{path_to_dir}");
    Ok(())
}

