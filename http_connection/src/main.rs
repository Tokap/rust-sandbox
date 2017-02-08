#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_rustls;
extern crate uuid;
extern crate url;
extern crate config;

use std::error::Error;
use hyper::{Client, Url};
use hyper::net::HttpsConnector;
use hyper::header::{Headers, Authorization, Basic};
use serde_json::Value;
use std::io::Read;


type Outcome = Result<Value, String>;

const root: &'static str = "https://jsonplaceholder.typicode.com";
const full_url: &'static str = "https://jsonplaceholder.typicode.com/posts/1";
const multi_post_url: &'static str = "https://jsonplaceholder.typicode.com/posts";
const slugger: &'static str = "/posts/1";


#[allow(dead_code)]
fn client() -> Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}


pub struct UrlBuilder {
    base_url: String,
    slug: String,
}


impl UrlBuilder {

    pub fn new(base_url: &str, slug: &str) -> UrlBuilder {
        UrlBuilder {
            base_url: base_url.to_string(),
            slug: slug.to_string(),
        }
    }

    pub fn compile(&self) -> hyper::Url {
        let mut url = hyper::Url::parse(self.base_url.as_ref()).unwrap();

        url.set_path(self.slug.as_ref());

        url
    }
}

#[get("/")]
fn index() -> &'static str {
    "Main Page with Nothing Interesting!"
}

#[get("/howdy")]
fn howdy() -> &'static str {
    "Hello, there sir!"
}


#[get("/reviews")]
fn reviews() -> String {
    match call(multi_post_url) {
        Ok(r) => return r.to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

#[get("/reviews/single")]
fn review() -> String {
    match call(full_url) {
        Ok(r) => return r.to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

fn start_server() {
    rocket::ignite()
    .mount("/", routes![index, howdy, review, reviews])
    .launch();
}


// Simply change expected type and this works with strings as well
fn call(url: &str) -> Outcome {
    client()
        .get(url)
        // .body(body)
        .send()
        .map_err(|x| format!("{:?}", x))
        .map(|x| {
            print!("RES: {:?}", &x);
            x
        })
        .and_then(|r| serde_json::from_reader(r).map_err(|x| format!("{:?}", x)))
}


fn main() {
    start_server();
}
