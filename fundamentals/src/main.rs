#![feature(use_extern_macros)]

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;

// extern crate rustc_serialize;
// use rustc_serialize::json;

use serde_json::*;
use serde::{Serialize, Deserialize};
// use std::ops::{Carrier};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SomePerson {
    name: String,
    age: u32,
}

fn main() {
    let john = json!({
      "name": "John Doe",
      "age": 43,
    });

    let j = serde_json::to_string(&john);

    println!("Encoded!!: {:?}", j );

}
