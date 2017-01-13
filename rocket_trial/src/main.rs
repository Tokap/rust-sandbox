#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/howdy")]
fn howdy() -> &'static str {
    "Hello, there sir!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/hello/<name>/<age>/<cool>")]
fn cool(name: &str, age: u8, cool: bool) -> String {
    if cool {
      format!("You're a cool {} year old, {}!", age, name)
    } else {
      format!("{}, we need to talk about your coolness.", name)
    }
}


fn main() {
    rocket::ignite()
    .mount("/", routes![cool, howdy])
    .launch();
    // rocket::ignite().mount("/", routes![howdy]).launch();
}
