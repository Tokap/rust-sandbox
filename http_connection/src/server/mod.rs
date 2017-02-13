extern crate rocket;

use connections::call;
use helpers::return_truncated_json;

const FULL_URL: &'static str        = "https://jsonplaceholder.typicode.com/posts/1";
const MULTI_POST_URL: &'static str  = "https://jsonplaceholder.typicode.com/posts";
// const ROOT: &'static str            = "https://jsonplaceholder.typicode.com";
// const SLUGGER: &'static str         = "/posts/1";

#[get("/")]
pub fn index() -> &'static str {
    "Main Page with Nothing Interesting!"
}

#[get("/howdy")]
pub fn howdy() -> &'static str {
    "Hello, there sir!"
}


#[get("/reviews")]
pub fn reviews() -> String {
    match call(MULTI_POST_URL) {
        Ok(r) => return r.to_string(),
        Err(e) => return format!("No Reviews Found {:?}", e)
    }
}

#[get("/reviews/single")]
pub fn review() -> String {
    match call(FULL_URL) {
        Ok(r) => return r.to_string(),
        Err(_) => return format!("No Reviews Found")
    }
}

#[get("/reviews/<num>")]
pub fn review_count(num: usize) -> String {
    let q;
    match call(MULTI_POST_URL) {
        Ok(r) => q = r,
        Err(_) => return format!("No Reviews Found")
    }
    return_truncated_json(q, num)
}


pub fn start_server() {
    rocket::ignite()
    .mount("/", routes![index, howdy, review, reviews, review_count])
    .launch();
}
