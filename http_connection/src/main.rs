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

type Outcome = Result<Value, String>;

const root: &'static str = "https://jsonplaceholder.typicode.com";


#[allow(dead_code)]
fn client() -> Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

// #[derive(Debug)]
// struct MyUrl {
//
// }

pub struct Builder {
    base_url: String,
    slug: String,
    // full_url: String,
    // username: String,
    // password: String,
}


impl Builder {
    // pub fn new(base_url: String, slug: String) -> Builder {
    //     Builder {
    //         base_url: base_url,
    //         slug: slug,
    //     }
    // }

    pub fn new(base_url: &str, slug: &str) -> Builder {
        Builder {
            base_url: base_url.to_string(),
            slug: slug.to_string(),
        }
    }

    pub fn compile(&self) -> hyper::Url {
        let mut url = hyper::Url::parse(self.base_url.as_ref()).unwrap();

        url.set_path(req_type.path(self.slug.as_ref()).as_ref());

        url
    }
}


fn merge_url(base: &str, slug: &str) -> String {
    let fixed_base = base.to_string();
    return fixed_base + slug;
}


#[allow(dead_code)]
fn call(url: Url, body: &str) -> Outcome {
    client()
        .post(url)
        .body(body)
        .send()
        .map_err(|x| format!("{:?}", x))
        .map(|x| {
            print!("RES: {:?}", &x);
            x
        })
        .and_then(|r| serde_json::from_reader(r).map_err(|x| format!("{:?}", x)))
}

fn main() {




    let my_url = merge_url(root, "/more/words");

    call(my_url, "");

    println!("Hello, world!");
    // println!("{}", my_url)
}
