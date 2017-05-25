extern crate serde;
// extern crate serde_json;

extern crate toml;
use std::collections::BTreeMap;

use std::fs::File;
use std::env;
use std::io::{Result, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorization {
    pub origins: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiInfo {
    pub provider: String,
    pub url: String,
    pub params: Vec<String>,
    pub format: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiCollection {
    pub authorization: Authorization,
    pub api: BTreeMap<String, Vec<ApiInfo>>
}

pub fn load_config() -> Result<ApiCollection> {
    let config_file = env::current_dir()?.join("config/apis.toml");
    println!("Load {}", config_file.display());

    let mut input = String::new();
    let mut file = File::open(&config_file)?;
    file.read_to_string(&mut input)?;

    // let deserialized: ApiCollection = serde_json::from_str(&input).unwrap();
    let deserialized: ApiCollection = toml::from_str(&input).unwrap();
    // println!(" -> {:?}", deserialized);
    return Ok(deserialized);
}
