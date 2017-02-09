use hyper::{Client};
use hyper::net::HttpsConnector;
use serde_json;
use serde_json::Value;
use hyper_rustls;
// use hyper::header::{Headers, Authorization, Basic};

type Outcome = Result<Value, String>;

pub fn call(url: &str) -> Outcome {
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
pub fn client() -> Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}
