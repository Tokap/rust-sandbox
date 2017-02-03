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
use serde_json::Value;

fn main() {
    println!("Hello, world!");
}
