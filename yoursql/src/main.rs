#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

extern crate mysql;

extern crate url;
extern crate net2;

use mysql::{Pool};

use url::{Url, ParseError};

mod patsql;

use patsql::write::{json_write_to_table, simple_json_insert, write_to_table};
use patsql::pool::{build_basic_pool, build_pool_json, connection_is_active, build_basic_pool_with_tcp_test};
use patsql::read::{get_by_param, get_by_two_params, get_by_raw};
// use patsql::conntest;
// use patsql::error::*;

use std::sync::Arc;
// use std::time::Duration;
// use std::net::TcpStream;

use net2::TcpBuilder;

use std::io::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};



extern crate nix;
use std::net::TcpStream;
use std::time::Duration;
use std::os::unix::io::FromRawFd;

// mod error;
// pub use error::ConnectionError;


fn main() {
    // let pool: Pool = build_basic_pool_with_tcp_test("10.101.0.1", "ip_dashboard", "root", "", 20133, 5).unwrap();
    let dud_pool: Result<Pool,String> = build_basic_pool("Bob", "ip_brolytics", "root", "", 3306);

    println!("Pool Return Looks Like: {:?}", pool);
}

// let five_seconds = Duration::new(5, 0);
// let ip_address = IpAddr::V4(Ipv4Addr::new(10, 101, 0, 1));
// let socket = SocketAddr::new(ip_address, 20133);
//
// let final_connection = tcp_connect_with_timeout(socket, five_seconds);
//
// println!(" Some Words! Yay! {:?}", final_connection.is_err());





// ORIGINAL NOTES BELOW:
/*
let five_seconds = Duration::new(5, 0);
let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();
// let stream = TcpStream::connect("10.101.0.1:20133").unwrap();
stream.set_read_timeout(Some(five_seconds));

// if let Ok(stream) = TcpStream::connect("10.101.0.1:20133") {
//     println!("Connected to the server!");
// } else {
//     println!("Couldn't connect to server...");
// }

// let db_url =  "localhost:3306";
// let db_url =  "mysql://root@localhost:3306/ip_brolytics";
// let alpha_vpn =  "10.101.0.1:20133";

let tcp = TcpBuilder::new_v4().unwrap();
tcp.reuse_address(true).unwrap();

// let mut stream = tcp.connect(alpha_vpn).unwrap();
// let mut stream = tcp.connect("127.0.0.1:80").unwrap();
// let table: String = format!("status");

println!(" TCP Connection Pool Result: {:?}", stream.read_timeout());



// POOL HANGING TRIGGERS:
// let test_json: String = "{\"network\":\"twitter\",\"network_id\":\"1713789122\",\"network_username\":\"buck_johnson\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\"}".to_string();
// let json_insert: Result<String, String> = simple_json_insert(table, test_json);
//
// println!("TEST JSON {:?}", json_insert);
//
// let simple_json = String::from(r#" {"status": "jumbled_af"}"#) ;
// let json_insert: String = simple_json_insert(table, simple_json).unwrap();


// let pool: Pool = build_basic_pool("127.0.0.1", "ip_brolytics", "root", "", 3306).unwrap();
// let dud_pool: Result<Pool,String> = build_basic_pool("Bob", "ip_brolytics", "root", "", 3306);
// let new_pool: Pool = build_pool_json(pool_json.dump());

// let trial = dud_pool.try_get_conn(5000);
// println!("Is Pool Trial from Main Error: {:?}", dud_pool); */
