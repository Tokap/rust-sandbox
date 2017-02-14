#![feature(use_extern_macros)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate mysql;

extern crate rocket;
extern crate hyper;
extern crate hyper_rustls;
extern crate uuid;
extern crate url;
extern crate config;

use mysql::{OptsBuilder, Pool, PooledConn, Error, Value, QueryResult};
use serde_json::*;
use serde::{Serialize, Deserialize};

use mysql::value::from_row;

mod server;
mod connections;
mod helpers;
mod yoursql;

/*******************************************************/
/********************* My Structs *********************/
/*****************************************************/

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SomePerson {
    name: String,
    age: u32,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountDataArchive {
    id: String,
    network: String,
    network_id: String,
    network_username: String,
    follower_count: String,
    correlation_id: String,
    archive_id: String,
    type_id: String,
    created_at: String,
    updated_at: String,
    deleted: String,
    deleted_timestamp: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountDataWrite {
    network: String,
    network_id: String,
    network_username: String,
    follower_count: String,
    correlation_id: String,
    archive_id: String,
    type_id: String,
}

// Clyde code: Awesome && thank you!!
impl AccountDataWrite {
    fn get_field_names(&self) -> Vec<String> {

        let mut field_names: Vec<String> = Vec::new();
        let temp = json!(AccountDataWrite::default());
        let er =
            if let serde_json::Value::Object(o) = temp { o }
            else {unreachable!()};
        for (key, _) in er {
            field_names.push(key);
        }
        field_names
    }
}

/*******************************************************/
/******************* Tests & Sandbox ******************/
/*****************************************************/
fn main() {
    let pool: Pool = yoursql::build_pool("127.0.0.1", "ip_brolytics", "root", 3306);

    let table: String = String::from("account_data_archive");
    
    let mut tuple_vec: Vec<(String, String)> = Vec::new();
        tuple_vec.push(("network".to_string(), "twitter".to_string()));
        tuple_vec.push(("network_id".to_string(), "8964323".to_string()));
        tuple_vec.push(("network_username".to_string(), "jimbo".to_string()));
        tuple_vec.push(("follower_count".to_string(), "456733".to_string()));
        tuple_vec.push(("correlation_id".to_string(), "89J6X43C23".to_string()));
        tuple_vec.push(("archive_id".to_string(), "1".to_string()));
        tuple_vec.push(("type_id".to_string(), "2".to_string()));

    yoursql::basic_write_to_table(table, tuple_vec, pool);
    // yoursql::get_account_json_by_id("1", "account_data_archive", pool);

    let bill: SomePerson = SomePerson {
        name: "Patrick".to_string(),
        age: 31
    };

    // println!("Bill's Actual Name: {:?}", bill.name);
}
