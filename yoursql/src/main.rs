#![feature(use_extern_macros)]

#[macro_use]
extern crate json;

extern crate mysql;

use mysql::{Pool};

mod patsql;

fn main() {

    let test_json = "{\"id\":\"1\",\"network\":\"twitter\",\"network_id\":\"17137891\",\"network_username\":\"walmart\",\"follower_count\":\"851196\",\"correlation_id\":\"97a2e620-6d45-46bb-bf16-5c2c6082897d\",\"archive_id\":\"2\",\"type_id\":\"2\",\"created_at\":\"2017-02-01 11:34:56\",\"updated_at\":\"2017-02-01 11:34:56\",\"deleted\":\"0\",\"deleted_timestamp\":\"NULL\"}";

    let pool: Pool = patsql::build_pool("127.0.0.1", "ip_brolytics", "root", 3306);

    let table: String = String::from("account_data_archive");

    let mut tuple_vec: Vec<(String, String)> = Vec::new();
        tuple_vec.push(("network".to_string(), "twitter".to_string()));
        tuple_vec.push(("network_id".to_string(), "8964323".to_string()));
        tuple_vec.push(("network_username".to_string(), "jimbo".to_string()));
        tuple_vec.push(("follower_count".to_string(), "456733".to_string()));
        tuple_vec.push(("correlation_id".to_string(), "89J6X43C23".to_string()));
        tuple_vec.push(("archive_id".to_string(), "1".to_string()));
        tuple_vec.push(("type_id".to_string(), "2".to_string()));

    patsql::simple_json_insert(table, test_json.to_string());    

    // patsql::basic_write_to_table(table, tuple_vec, pool);
    let return_value: String = patsql::get_by_param("id", "1", "account_data_archive", pool);
    println!("My Outcome Looks Like: {:?}",return_value);
}
