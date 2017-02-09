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

mod server;
mod connections;
mod helpers;

#[derive(Debug)]
pub struct ReturnContent {
    body: String,
    id: usize,
    title: String,
    userId: usize
}

fn main() {
    server::start_server();
}
