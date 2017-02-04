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
const slugger: &'static str = "/posts/1";


#[allow(dead_code)]
fn client() -> Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}


pub struct Builder {
    base_url: String,
    slug: String,
    // username: String,
    // password: String,
}


impl Builder {

    pub fn new(base_url: &str, slug: &str) -> Builder {
        Builder {
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


fn merge_url(base: &str, slug: &str) -> String {
    let fixed_base = base.to_string();
    return fixed_base + slug;
}


fn call(url: Url) -> Outcome {
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

// pub fn lets_match(input: Outcome) -> String {
//     match input {
//         input =>input.to_string(),
//         None => "Booo"
//     }
// }

fn main() {

    // let mut my_string = String::new();
    let mut my_string = "words".to_string();
    // my_string = "words".to_string();

    let builder = Builder::new(root, slugger);
    let compiled = builder.compile();

    let my_url = merge_url(root, "/more/words");

    let tester = call(compiled);

    // call(my_url);

    println!("Hello, world!");
    println!("{}", my_url);
    println!("{:?}", tester);
    println!("{:?}", my_string)
    // println!("{}", compiled)
}
