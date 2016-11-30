#![feature(static_in_const)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate rustc_serialize;

extern crate hyper;
use hyper::client::Client;
use hyper::header::{AccessControlAllowOrigin, ContentType, Origin};

#[macro_use] extern crate nickel;
use nickel::{Nickel, QueryString, HttpRouter};
use nickel::status::StatusCode::{self};

extern crate uritemplate;
use uritemplate::UriTemplate;

extern crate time;
use std::io::Read;

mod config;
use config::{ApiInfo};

static GREETING : &str = "Starting API proxy...";

fn process_api(api: &ApiInfo, query: &nickel::Query, response: &mut nickel::Response) -> (StatusCode, String) {
    let mut uri = UriTemplate::new(api.url.as_str());
    for param in api.params.as_slice() {
        uri.set(param, query.get(param).unwrap());
    }
    let url = uri.build();
    println!(" -> {}", url);

    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    if let Some(ref format) = api.format {
        let content_type = match format.as_str() {
            "json" => ContentType::json(),
            _ => ContentType::html()
        };
        response.set(content_type);
    }
    (StatusCode::Ok, buffer)
}

fn select_and_process_api(apis: &Vec<ApiInfo>, request: &mut nickel::Request, response: &mut nickel::Response) -> (StatusCode, String) {
    let query = request.query();
    if let Some(provider) = query.get("provider") {
        if let Some(api) = apis.iter().find({|item| item.provider == provider}) {
            process_api(api, query, response)
        } else {
            (StatusCode::BadRequest, format!("Provider {} not found", provider))
        }
    } else {
        (StatusCode::BadRequest, "No provider parameter".to_string())
    }
}

fn get_origin(request: &nickel::Request) -> String {
    if let Some(origin) = request.origin.headers.get::<Origin>() {
        let origin = if let Some(port) = origin.host.port {
            format!("{}://{}:{}", origin.scheme, origin.host.hostname, port)
        } else {
            format!("{}://{}", origin.scheme, origin.host.hostname)
        };
        return origin;
    } else {
        return String::new();
    }
}

#[allow(unused_must_use)]
fn main() {
    println!("{}", GREETING);

    let mut server = Nickel::new();
    let config = config::load_config();

    for (name, apis) in config.api {
        let name = name.to_owned();
        let allowed_origins = config.authorization.origins.clone();

        server.get(format!("/{}", name), middleware!{ |request, mut response|
            println!("{} {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), request.origin.uri);
            
            let origin = get_origin(request);
            if allowed_origins.contains(&origin) {
                response.set(AccessControlAllowOrigin::Value(origin));
                select_and_process_api(&apis, request, &mut response)
            } else {
                (StatusCode::Unauthorized, "Origin not allowed".to_string())
            }
        });
    }

    server.listen("0.0.0.0:6767");
}
