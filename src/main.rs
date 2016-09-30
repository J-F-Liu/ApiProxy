#![feature(static_in_const)]

extern crate hyper;
use hyper::client::Client;
use hyper::header::{AccessControlAllowOrigin, ContentType};

#[macro_use] extern crate nickel;
use nickel::{Nickel, QueryString, HttpRouter};
use nickel::status::StatusCode::{self};

extern crate toml;
use toml::Value;

extern crate time;
use std::io::Read;

mod config;

static GREETING : &str = "Starting API proxy...";

fn process_api(detail: &toml::Table, query: &nickel::Query, response: &mut nickel::Response) -> (StatusCode, String) {
    let mut params = detail["params"].as_slice().unwrap().into_iter().map(|param| query.get(param.as_str().unwrap()).unwrap());
    // let url = format!(detail.url, params);
    let url = detail["url"].as_str().unwrap().replace("{}", params.next().unwrap());
    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    response
      .set(AccessControlAllowOrigin::Any)
      .set(ContentType::json());
    (StatusCode::Ok, buffer)
}

fn select_and_process_api(list: &toml::Array, request: &mut nickel::Request, response: &mut nickel::Response) -> (StatusCode, String) {
    let query = request.query();
    if let Some(provider) = query.get("provider") {
        if let Some(detail) = list.iter().find({|api| api.as_table().unwrap()["provider"].as_str().unwrap() == provider}) {
            process_api(detail.as_table().unwrap(), query, response)
        } else {
            (StatusCode::BadRequest, format!("Provider {} not found", provider))
        }
    } else {
        (StatusCode::BadRequest, "No provider parameter".to_string())
    }
}

#[allow(unused_must_use)]
fn main() {
    println!("{}", GREETING);

    let mut server = Nickel::new();
    let config = config::load_config();
    let apis = config["Api"].as_table().unwrap();

    for (api, details) in apis {
        let api = api.to_owned();
        let details = details.to_owned();
        server.get(format!("/{}", api), middleware!{ |request, mut response|
            println!("{} {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), request.origin.uri);
            match &details {
                &Value::Table(ref detail) => process_api(detail, request.query(), &mut response),
                &Value::Array(ref list) => select_and_process_api(list, request, &mut response),
                _ => (StatusCode::BadRequest, "Error in api configuration".to_string())
            }
        });
    }

    server.listen("0.0.0.0:6767");
}
