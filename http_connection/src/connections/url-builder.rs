use hyper::{Client, Url};
use hyper::net::HttpsConnector;
use hyper::header::{Headers, Authorization, Basic};

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
