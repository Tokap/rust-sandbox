// Can Call on a Pool to get a connection:
fn get_conn(&self) -> MyResult<PooledConn>
//
// Gives you a PooledConn.
// Pool will check that connection is alive via
// Conn::ping and will call Conn::reset if necessary.

// Building a simple non-generic connection:
fn build_pool() -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .db_name(Some("ip_brolytics"))
        .user(Some("root"));
    let pool = Pool::new(builder);
    pool.unwrap()
}
// NOTE: Pools can be cloned with .clone() & debug is implemented


// Testing connection:
fn test_and_output_connection(p: Pool) -> () {
    match p.try_get_conn(50000) {
        Ok(rez) => println!("Okay! {:?}", rez),
        Err(e) => println!("Bad news: {:?}", e),
    }
}
/********************************/
/* BUILD AS OF MONDAY - FEB 13 */
/******************************/

#![feature(use_extern_macros)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate rocket;
extern crate hyper;
extern crate hyper_rustls;
extern crate uuid;
extern crate url;
extern crate config;
extern crate mysql;

use mysql::{OptsBuilder, Pool, PooledConn, Error, Value};
use serde_json::*;
use serde::{Serialize, Deserialize};

mod server;
mod connections;
mod helpers;

// Serialize, Deserialize
// RustcDecodable, RustcEncodable,
// PartialEq, Eq,

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

// Clyde code: Awesome && thank you!!
impl AccountDataArchive {
    pub fn get_field_names(&self) -> Vec<String> {
        let mut field_names: Vec<String> = Vec::new();
        let temp = json!(AccountDataArchive::default());
        let er =
            if let serde_json::Value::Object(t) = temp { t } // I believe this is a clever obj conversion
            else {unreachable!()};
        for (key, _) in er {
            field_names.push(key);
        }
        field_names
    }
}
// Notes about unreachable!:
// A utility macro for indicating unreachable code.
// This is useful any time that the compiler can't determine that some code is unreachable. For example:
// - Match arms with guard conditions.
// - Loops that dynamically terminate.
// - Iterators that dynamically terminate.

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


fn build_pool(hostname: &str, db_name: &str, user: &str, port: u16) -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(Some(hostname))
        .tcp_port(port)
        .db_name(Some(db_name))
        .user(Some(user));
    let pool = Pool::new(builder);
    pool.unwrap()
}

fn test_and_output_connection(p: Pool) -> bool {
    match p.try_get_conn(50000) {
        Ok(rez) => true,
        Err(e) => false,
    }
}


pub fn get_account_data_by_param(param: &str, identifier: &str, pool: Pool) -> Vec<AccountDataArchive> {

    let mut conn = pool.get_conn().unwrap(); // Get a connection to the pool

    let mut all_returns: Vec<Vec<String>> = Vec::new();
    let mut account_data_vec: Vec<AccountDataArchive> = Vec::new();
    let params = format!("SELECT * FROM `account_data_archive` WHERE `{}`={}", param, identifier);

    conn.query(params).map(|query_result| {

         for row in query_result {
            let unwrapped =  row.unwrap().unwrap();
            let mut row_returns: Vec<String> = Vec::new();

            for value in unwrapped {
                row_returns.push(value.into_str())
            }

            all_returns.push(row_returns);
        }

        for row_data in all_returns {
            let account_info = AccountDataArchive {
                id: row_data[0].to_string().to_owned(),
                network: row_data[1].to_string().to_owned(),
                network_id: row_data[2].to_string().to_owned(),
                network_username: row_data[3].to_string().to_owned(),
                follower_count: row_data[4].to_string().to_owned(),
                correlation_id: row_data[5].to_string().to_owned(),
                archive_id: row_data[6].to_string().to_owned(),
                type_id: row_data[7].to_string().to_owned(),
                created_at: row_data[8].to_string().to_owned(),
                updated_at: row_data[9].to_string().to_owned(),
                deleted: row_data[10].to_string().to_owned(),
                deleted_timestamp: row_data[11].to_string().to_owned(),
            };
            account_data_vec.push(account_info);
        }
        println!("Final Return: {:?}", account_data_vec);
    });
    account_data_vec
}


pub fn write_account_data(pool: Pool, data: AccountDataWrite) -> () {

    let mut conn = pool.get_conn().unwrap();

    let params = format!("INSERT INTO `account_data_archive`
        (network, network_id, network_username,
         follower_count, correlation_id, archive_id, type_id)
         VALUES ('{}', '{}', '{}', '{}', '{}', '{}','{}')",
         data.network, data.network_id, data.network_username, data.follower_count,
         data.correlation_id, data.archive_id, data.type_id);

    conn.query(params).map(|query_result| {
         for row in query_result {
            let unwrapped =  row.unwrap().unwrap();
            let row_returns: Vec<String> = Vec::new();
        }
    }); // Currently, no error handling or confirmation of return
}

pub fn get_account_data_by_id(identifier: &str, pool: Pool) -> Vec<AccountDataArchive> {
    get_account_data_by_param("id", identifier, pool)
}

fn main() {
    let x = AccountDataWrite::default();

    let trial_run: AccountDataWrite = AccountDataWrite {
        network: "twitter".to_string(),
        network_id: "8675309".to_string(),
        network_username: "imgenerallyawesome".to_string(),
        follower_count: "1234567".to_string(),
        correlation_id: "12XD56HY34".to_string(),
        archive_id: "1".to_string(),
        type_id: "2".to_string(),
    };

    let dos: AccountDataWrite = trial_run.clone();
    let tres: AccountDataWrite = trial_run.clone();

    let qq = serde_json::to_string(&trial_run);

    let json_string = json!({
        "network": "twitter",
        "network_id": "123456",
        "network_username": "funky_trunks",
        "follower_count": "12345",
        "correlation_id": "X12Y3456",
        "archive_id": "2",
        "type_id": "2",
    });


    let pool: Pool = build_pool("127.0.0.1", "ip_brolytics", "root", 3306);
    get_account_data_by_id("1", pool.clone());
    // write_account_data(pool.clone(), trial_run);

    println!("AccountData Review: {:?}", trial_run.network);
    println!("Default Serialize: {:?}", qq);
    // server::start_server();

    let john = json!({
      "name": "John Doe",
      "age": 43,
    });

    let j = serde_json::to_string(&john).unwrap();

    println!("Finally Encoded!!: {:?}", j );
    println!("Check Fields!!!: {:?}", dos.get_field_names());

}


// Other Stuff:
// Clyde code: Awesome && thank you!!
impl AccountDataArchive {
    pub fn get_field_names(&self) -> Vec<String> {
        let mut field_names: Vec<String> = Vec::new();
        let temp = json!(AccountDataArchive::default());
        let er =
            if let serde_json::Value::Object(t) = temp { t } // I believe this is a clever obj conversion
            else {unreachable!()};
        for (key, _) in er {
            field_names.push(key);
        }
        field_names
    }
}
// Notes about unreachable!:
// A utility macro for indicating unreachable code.
// This is useful any time that the compiler can't determine that some code is unreachable. For example:
// - Match arms with guard conditions.
// - Loops that dynamically terminate.
// - Iterators that dynamically terminate.
