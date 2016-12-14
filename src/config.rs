extern crate serde;
// extern crate serde_json;
// use self::serde_json::value::{Map};

extern crate toml;
use std::collections::BTreeMap;

use std::fs::File;
use std::env;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, RustcDecodable)]
pub struct  Authorization {
    pub origins: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, RustcDecodable)]
pub struct  ApiInfo {
    pub provider: String,
    pub url: String,
    pub params: Vec<String>,
    pub format: Option<String>
}

#[derive(Serialize, Deserialize, Debug, RustcDecodable)]
pub struct ApiCollection {
    pub authorization: Authorization,
    pub api: BTreeMap<String, Vec<ApiInfo>>
}

pub fn load_config() -> ApiCollection {
    let mut input = String::new();
    let config_file = env::current_dir().unwrap().join("config/apis.toml");
    println!("Load {}", config_file.display());

    File::open(&config_file).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    // let deserialized: ApiCollection = serde_json::from_str(&input).unwrap();
    let deserialized: ApiCollection = toml::decode_str(&input).unwrap();
    // println!(" -> {:?}", deserialized);
    return deserialized;
}
