// #[macro_use]
// extern crate json;
// extern crate jsonsql;

extern crate diesel;
use diesel::mysql::MysqlConnection;

extern crate dotenv;
// use jsonsql::pool::{Pool, build_basic_pool};



// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct SomePerson {
//     name: String,
//     age: u32,
// }

use dotenv::dotenv;
use std::env;
use diesel::prelude::*; // Need this to make the entire conn process work.

fn main() {
    dotenv().ok();
    let db_url = "mysql://root@bill/ip_brolytics";
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = MysqlConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", database_url));
    // let x = MysqlConnection::establish(db_url);

    println!("DB Url: {:?}", database_url);

}

/*
let test_json: String = "{\"network\":\"twitter\",\"network_id\":\"1713789122\",\"network_username\":\"buck_johnson\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\"}".to_string();

let pool: Pool = jsonsql::pool::build_basic_pool("127.0.0.1", "ip_brolytics", "root", "", 3306);
// let new_pool: Pool = build_basic_pool_with_tcp_test("127.0.0.1", "ip_brolytics", "root", "", 3306, 5).unwrap();
// let pool: Pool = build_basic_pool_with_tcp_test("10.101.0.1", "ip_dashboard", "root", "", 20133, 15).unwrap();

println!("New Pool Test: {:?}", pool);

let table: String = String::from("account_data_archive");

// let print_statement: String = patsql::simple_json_insert(table, test_json);
// println!("My Print String Looks Like: {:?}",print_statement);

// let my_return = json_write_to_table(table, test_json, pool);
// println!("My RETURN: {:?}", my_return);
// let mut parsed = json::parse(&test_json).unwrap();
// let q = parsed["network"].to_string();
// println!("Q Test: {:?}", q);
//
// let return_value: Result<String, String> = jsonsql::read::get_by_param("id", "1", "account_data_archive", pool);
// println!("My Outcome Looks Like: {}",return_value.unwrap());
*/
