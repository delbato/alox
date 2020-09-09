extern crate alox;

use std::{
    result::Result as StdResult,
    error::Error,
    fs::File,
    io::Read
};

use alox::{
    config::{
        AloxConfig
    },
    toml::from_str
};

type Result<T> = StdResult<T, Box<dyn std::error::Error>>;


#[test]
fn test_config_alox() -> Result<()> {
    println!("CWD: {}", std::env::current_dir().unwrap().to_str().unwrap());
    let mut file = File::open("../test-data/alox.toml")?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let alox_config: AloxConfig = from_str(&file_contents)?;
    println!("{:#?}", alox_config);
    Ok(())
}
