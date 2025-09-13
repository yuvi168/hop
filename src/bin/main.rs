use std::error::Error;
use clap::{Arg, Command, ArgAction};
use hop::query_database::get_query;
fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("refresh")
        .arg(Arg::new("list")
            .short('l')
            .action(ArgAction::Count))
        .arg(Arg::new("migrations")
            .short('m')
            .action(ArgAction::Count))
        .arg(Arg::new("path")
            .short('p')
            .required(true)
            .action(ArgAction::Set))
        .get_matches();
    if matches.get_count("list") == 1 {
        hop::retrieve::retrieve_zsh_history()?;
    }
    if matches.get_count("migrations") == 1 {
        hop::run_migration();
    }
    if let Some(path) = matches.get_one::<String>("path") {
        let path = get_query::get_path(path.clone())?;
        println!("{path}");
    }
    Ok(())
}

