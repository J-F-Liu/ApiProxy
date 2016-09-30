#![feature(static_in_const)]

extern crate hyper;
use hyper::client::Client;
use hyper::header::{AccessControlAllowOrigin, ContentType, Origin};

#[macro_use] extern crate nickel;
use nickel::{Nickel, QueryString, HttpRouter};
use nickel::status::StatusCode::{self};

extern crate toml;
use toml::Value;

extern crate uritemplate;
use uritemplate::UriTemplate;

extern crate time;
use std::io::Read;

mod config;

static GREETING : &str = "Starting API proxy...";

fn process_api(api: &toml::Table, query: &nickel::Query, response: &mut nickel::Response) -> (StatusCode, String) {
    let params = api["params"].as_slice().unwrap().into_iter().map(|param| param.as_str().unwrap());
    let mut uri = UriTemplate::new(api["url"].as_str().unwrap());
    for param in params {
        uri.set(param, query.get(param).unwrap());
    }
    let url = uri.build();
    println!(" -> {}", url);

    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    if let Some(format) = api.get("format").and_then({|v|v.as_str()}) {
        let content_type = match format {
            "json" => ContentType::json(),
            _ => ContentType::html()
        };
        response.set(content_type);
    }
    (StatusCode::Ok, buffer)
}

fn select_and_process_api(list: &toml::Array, request: &mut nickel::Request, response: &mut nickel::Response) -> (StatusCode, String) {
    let query = request.query();
    if let Some(provider) = query.get("provider") {
        let mut apis = list.iter().map({|item| item.as_table().unwrap()});
        if let Some(api) = apis.find({|item| item["provider"].as_str().unwrap() == provider}) {
            process_api(api, query, response)
        } else {
            (StatusCode::BadRequest, format!("Provider {} not found", provider))
        }
    } else {
        (StatusCode::BadRequest, "No provider parameter".to_string())
    }
}

fn get_origin(request: &nickel::Request) -> String {
    let origin = request.origin.headers.get::<Origin>().unwrap();
    let origin = if let Some(port) = origin.host.port {
        format!("{}://{}:{}", origin.scheme, origin.host.hostname, port)
    } else {
        format!("{}://{}", origin.scheme, origin.host.hostname)
    };
    return origin;
}

#[allow(unused_must_use)]
fn main() {
    println!("{}", GREETING);

    let mut server = Nickel::new();
    let config = config::load_config();
    let apis = config["Api"].as_table().unwrap();
    let allowed_origins = config["Authorization"].lookup("AllowOrigin").unwrap().as_slice().unwrap().to_owned();

    for (name, details) in apis {
        let name = name.to_owned();
        let details = details.to_owned();
        let allowed_origins = allowed_origins.clone();

        server.get(format!("/{}", name), middleware!{ |request, mut response|
            println!("{} {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), request.origin.uri);
            let origin = get_origin(request);
            if allowed_origins.iter().any({|item| item.as_str().unwrap() == &origin}) {
                response.set(AccessControlAllowOrigin::Value(origin));
                match &details {
                    &Value::Table(ref api) => process_api(api, request.query(), &mut response),
                    &Value::Array(ref list) => select_and_process_api(list, request, &mut response),
                    _ => (StatusCode::BadRequest, "Error in api configuration".to_string())
                }
            } else {
                (StatusCode::Unauthorized, "Origin not allowed".to_string())
            }
        });
    }

    server.listen("0.0.0.0:6767");
}
