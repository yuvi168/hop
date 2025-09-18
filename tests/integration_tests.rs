use hop::query_database::get_query;
use std::error::Error;

#[test]
fn testing_path_finding() -> Result<(), Box<dyn Error>> {
        hop::run_migration();
        hop::retrieve::retrieve_zsh_history()?;
    let path = get_query::get_path(String::from("lcode/lrust"))?;
    assert_eq!(path, String::from("lcode/lrust"));
    Ok(())
}
