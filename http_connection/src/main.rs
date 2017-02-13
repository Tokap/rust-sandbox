#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;


extern crate rocket;
extern crate serde;
extern crate hyper;
extern crate hyper_rustls;
extern crate uuid;
extern crate url;
extern crate config;

extern crate mysql;
extern crate rustc_serialize;

use rustc_serialize::json;
use mysql::{OptsBuilder, Pool, PooledConn, Error, Value};

mod server;
mod connections;
mod helpers;

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnContent {
    id: u64,
    userId: u64,
    title: String,
    body: String,
}

#[derive(Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
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

pub struct AccountDataVector {
    account_data: Vec<AccountDataArchive>
}

// const POOL = mysql::Pool::new("").unwrap();

fn build_pool() -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .db_name(Some("ip_brolytics"))
        .user(Some("root"));
    let pool = Pool::new(builder);
    pool.unwrap() // should have proper error handling
}

fn test_and_output_connection(p: Pool) -> () {
    match p.try_get_conn(50000) {
        Ok(rez) => println!("Okay! {:?}", rez),
        Err(e) => println!("Bad news: {:?}", e),
    }
}


fn get_all_factory() -> Vec<AccountDataArchive> {

    let z = build_pool(); // Our pool. For this ex, build from static info shown above
    let mut conn = z.get_conn().unwrap(); // Get a connection to the pool

    let mut all_returns: Vec<Vec<String>> = Vec::new();
    let mut account_data_vec: Vec<AccountDataArchive> = Vec::new();

    conn.query("SELECT * FROM `account_data_archive`").map(|query_result| {

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

fn main() {
    get_all_factory();
}
