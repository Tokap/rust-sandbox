#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate jsonsql;

use jsonsql::pool::{Pool};

// extern crate rustc_serialize;
// use rustc_serialize::json;

// use serde_json::*;
use serde::{Serialize, Deserialize};
// use json::*;
// use mysql::{Pool};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SomePerson {
    name: String,
    age: u32,
}



fn main() {
    let test_json: String = "{\"network\":\"twitter\",\"network_id\":\"1713789122\",\"network_username\":\"buck_johnson\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\"}".to_string();

    let pool: Pool = jsonsql::pool::build_basic_pool("127.0.0.1", "ip_brolytics", "root", "", 3306);

    let table: String = String::from("account_data_archive");

    // let print_statement: String = patsql::simple_json_insert(table, test_json);
    // println!("My Print String Looks Like: {:?}",print_statement);

    // let my_return = json_write_to_table(table, test_json, pool);
    // println!("My RETURN: {:?}", my_return);
    let mut parsed = json::parse(&test_json).unwrap();
    let q = parsed["network"].to_string();
    println!("Q Test: {:?}", q);

    let return_value: Result<String, String> = jsonsql::read::get_by_param("id", "1", "account_data_archive", pool);
    println!("My Outcome Looks Like: {}",return_value.unwrap());

}
