#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;

// extern crate rustc_serialize;
// use rustc_serialize::json;

use serde_json::*;
use serde::{Serialize, Deserialize};
use json::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SomePerson {
    name: String,
    age: u32,
}

fn main() {
    // let john = json!({
    //   "name": "John Doe",
    //   "age": 43,
    // });

    /*** Some notes about using the Json library ***/
    let stringify = json::stringify("foobar cat".to_string()); // encapsulates in quotes
    assert_eq!(json::stringify(json::Null), "null"); // use of null
    let data = vec![1,2,3];
    assert_eq!(json::stringify(data), "[1,2,3]");

    // Adding Data to Objects
    let mut data = json::JsonValue::new_object();
    data["answer"] = 42.into();
    data["foo"] = "bar".into();
    assert_eq!(data.dump(), r#"{"answer":42,"foo":"bar"}"#);


    // My Stuff:
    let json_string = json::parse(r#"
    {
        "name": "john",
        "age": 25,
        "data": {
            "debt": 10000.00,
            "income": 400.00
        }
    }
    "#).unwrap();

    let instantiated = object!{
    "code" => 200,
    "success" => true,
    "payload" => object!{
        "features" => array![
            "awesome",
            "easyAPI",
            "lowLearningCurve"
            ]
        }
    };


    // let j = serde_json::to_string(&john);
    // let t = assert_eq!(json_string["data"]["debt"], 10000);

    println!("JSON Stringy: {:?}", stringify);

    println!("json String!!: {:?}", json_string["data"]["debt"] );
    println!("Object Creation!!: {:?}", instantiated );

}
