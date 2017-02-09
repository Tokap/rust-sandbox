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
// use hyper::header::{Headers, Authorization, Basic};
use serde_json::Value;


type Outcome = Result<Value, String>;

#[derive(Debug)]
pub struct ReturnContent {
    body: String,
    id: usize,
    title: String,
    userId: usize
}

const ROOT: &'static str            = "https://jsonplaceholder.typicode.com";
const FULL_URL: &'static str        = "https://jsonplaceholder.typicode.com/posts/1";
const MULTI_POST_URL: &'static str  = "https://jsonplaceholder.typicode.com/posts";
const SLUGGER: &'static str         = "/posts/1";

fn call(url: &str) -> Outcome {
    client()
        .get(url)
        .send()
        .map_err(|x| format!("{:?}", x))
        .map(|x| {
            print!("RES: {:?}", &x);
            x
        })
        .and_then(|r| serde_json::from_reader(r).map_err(|x| format!("{:?}", x)))
}

#[allow(dead_code)]
fn client() -> Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
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
    match call(MULTI_POST_URL) {
        Ok(r) => return r.to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

#[get("/reviews/single")]
fn review() -> String {
    match call(FULL_URL) {
        Ok(r) => return r.to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

#[get("/reviews/<num>")]
fn review_count(num: usize) -> String {
    let q;
    match call(MULTI_POST_URL) {
        Ok(r) => q = r,
        Err(_) => return format!("No Reviews Found")
    }
    return_truncated_json(q, num)
}

fn return_truncated_json(v: Value, i: usize) -> String {
    let vec;
    match v.as_array() {
        Some(r) => vec = r,
        None => return format!("Cannot Convert to vector!")
    }

    let my_slice = &vec[0..i];

    match serde_json::to_string(&my_slice) {
        Ok(r) => r,
        Err(_) => format!("Nothing to Return")
    }
}



fn start_server() {
    rocket::ignite()
    .mount("/", routes![index, howdy, review, reviews, review_count])
    .launch();
}

fn main() {
    start_server();
}
