#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

extern crate mysql;

use mysql::{Pool};

mod patsql;

use patsql::write::{json_write_to_table};
use patsql::pool::{build_basic_pool, build_pool_json, connection_is_active};
use patsql::read::{get_by_param, get_by_two_params, get_by_raw};

fn main() {

    let test_json: String = "{\"network\":\"twitter\",\"network_id\":\"1713789122\",\"network_username\":\"buck_johnson\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\"}".to_string();

    let pool_json = object!{
        "hostname" => "127.0.0.1",
        "port" => "",
        "db" => "ip_brolytics",
        "user" => "root",
        "password" => "",
        "socket" => ""
    };

    println!("TEST JSON {:?}", pool_json.dump());
    let pool: Pool = build_basic_pool("127.0.0.1", "ip_brolytics", "root", "", 3306);
    // let new_pool: Pool = build_pool_json(pool_json.dump());

    let table: String = String::from("account_data_archive");
    let sql: String = String::from("SELECT * FROM `account_data_archive`");
    let return_value: String = get_by_raw(sql, pool).unwrap();
    // let print_statement: String = patsql::simple_json_insert(table, test_json);
    // println!("My Print String Looks Like: {:?}",print_statement);

    // let return_value: String = get_by_param("id", "1", "account_data_archive", pool).unwrap();
    // let return_value: String = get_by_param("id", "2", "account_data_archive", pool).unwrap();
    println!("My Outcome Looks Like: {}",return_value);
}
