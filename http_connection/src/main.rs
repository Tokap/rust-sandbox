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
    match call(MULTI_POST_URL) {
        Ok(r) =>
            return r.as_array().unwrap().get(num).unwrap().to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

// fn test_function(v: Value) {
//
// }

fn start_server() {
    rocket::ignite()
    .mount("/", routes![index, howdy, review, reviews, review_count])
    .launch();
}

fn main() {
    // let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("Array Slice: {:?}", &xs[1..3+1]);

    start_server();
}
