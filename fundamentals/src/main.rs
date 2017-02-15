#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;
// extern crate jsonsql;
// use jsonsql as patsql;

// extern crate rustc_serialize;
// use rustc_serialize::json;

use serde_json::*;
use serde::{Serialize, Deserialize};
use json::*;

// use mysql::{Pool};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SomePerson {
    name: String,
    age: u32,
}



fn main() {
    // let test_json: String = "{\"network\":\"twitter\",\"network_id\":\"1713789122\",\"network_username\":\"buck_johnson\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\"}".to_string();
    //
    // let pool: Pool = build_pool("127.0.0.1", "ip_brolytics", "root", 3306);
    //
    // let table: String = String::from("account_data_archive");
    //
    // let mut tuple_vec: Vec<(String, String)> = Vec::new();
    //     tuple_vec.push(("network".to_string(), "twitter".to_string()));
    //     tuple_vec.push(("network_id".to_string(), "8964323".to_string()));
    //     tuple_vec.push(("network_username".to_string(), "jimbo".to_string()));
    //     tuple_vec.push(("follower_count".to_string(), "456733".to_string()));
    //     tuple_vec.push(("correlation_id".to_string(), "89J6X43C23".to_string()));
    //     tuple_vec.push(("archive_id".to_string(), "1".to_string()));
    //     tuple_vec.push(("type_id".to_string(), "2".to_string()));
    //
    // // let print_statement: String = patsql::simple_json_insert(table, test_json);
    // // println!("My Print String Looks Like: {:?}",print_statement);
    //
    // // let my_return = json_write_to_table(table, test_json, pool);
    // // println!("My RETURN: {:?}", my_return);
    //
    // let return_value: String = get_by_param_with_handling("id", "1", "account_data_archive", pool);
    // println!("My Outcome Looks Like: {}",return_value);

}
