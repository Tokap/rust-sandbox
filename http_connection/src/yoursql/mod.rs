#![feature(use_extern_macros)]

#![feature(plugin)]


use mysql::{OptsBuilder, Pool, PooledConn, Error, Value, QueryResult};
use serde_json::*;
use serde_derive::*;
use serde::{Serialize, Deserialize};

use mysql::value::from_row;

/*******************************************************/
/************** Creating a Pool Connection ************/
/*****************************************************/

// expand to allow for more options
pub fn build_pool(
    hostname: &str,
    db_name: &str,
    user: &str,
    port: u16) -> Pool {

        let mut builder = OptsBuilder::new();
        builder
            .ip_or_hostname(Some(hostname))
            .tcp_port(port)
            .db_name(Some(db_name))
            .user(Some(user));
        let pool = Pool::new(builder);
        pool.unwrap()
}

pub fn test_and_output_connection(p: Pool) -> bool {

    match p.try_get_conn(50000) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/*******************************************************/
/***************** Reading from mysql *****************/
/*****************************************************/

pub fn get_col_names(query_result: &QueryResult) -> Vec<String> {

    let column_hash_map = query_result.column_indexes(); // Split col names to hashmap
    let mut col_tuple_vec: Vec<(String, usize)> = Vec::new(); // Vec<(column_name, idx)>
    let mut col_name_vec: Vec<String> = Vec::new(); // Final Vec for return

    for (name, idx) in column_hash_map.iter() {
        col_tuple_vec.push( (name.to_string(), *idx) ) // make tuples w/ col name & index
    }
    col_tuple_vec.sort_by(|a,b| a.1.cmp(&b.1));

    for (column_name, _) in col_tuple_vec {
        col_name_vec.push(column_name);
    }
    col_name_vec
}

pub fn get_table_json_by_param(
    param: &str,
    identifier: &str,
    table: &str,
    pool: Pool,) -> Vec<String> {

        let mut conn = pool.get_conn().unwrap(); // Get a connection to the pool

        let mut all_row_values: Vec<Vec<String>> = Vec::new();
        let mut json_vec: Vec<String> = Vec::new();

        let params = format!("SELECT * FROM `{}` WHERE `{}`={}", table, param, identifier);

        conn.query(params).map(|query_result| {
            let col_name_vec: Vec<String> = get_col_names(&query_result);


            for row in query_result {
                let unwrapped = row.unwrap().unwrap();
                let mut row_returns: Vec<String> = Vec::new();

                for value in unwrapped {
                    row_returns.push(value.into_str())
                }

                all_row_values.push(row_returns);
            }

            for row_contents in all_row_values {
                let mut json_return: String = String::new(); // Where we build our JSON
                let col_names: Vec<String> = col_name_vec.clone();
                for i in 0..row_contents.len() {
                    let mut data: String = String::new();

                    if i == 0 { data.push('{') } // Add opening Object symbol on first iteration

                    data.push_str(&format!(r#" "{}": "{}""#,
                        col_names[i],
                        row_contents[i].replace("'", ""))
                    );

                    if i != row_contents.len() - 1 { data.push(','); } // Add comma if not last obj
                    else { data.push_str(" }"); } // Add closing symbols if last iteration

                    json_return.push_str(&data)
                }
                json_vec.push(json_return.clone()); //Add to our collection vector
            }
        });
         json_vec
    // altenatively, we can make the json string outside of the loop scope and return just that
    // Please note that such a method would not support conditions with multiple returns
}

/*******************************************************/
/******************* Writing to mysql *****************/
/*****************************************************/

// Should make a more tailored version that takes json
pub fn simple_insert_statement(
    table: String,
    params: Vec<(String, String)> ) -> String {

        let mut table_statement: String = format!("INSERT INTO `{}` ", table);
        let mut insert_keys: String = String::new();
        let mut insert_values: String = String::from(" VALUES ");


        for i in 0..params.len() {

            if i == 0 {
                insert_keys.push_str("( ");
                insert_values.push_str("( ");
            }

            if i == params.len() - 1 {
                insert_keys.push_str(&format!("{} ", params[i].0)); //key at first point in tuple
                insert_values.push_str(&format!("'{}' ", params[i].1)); // value in second
                insert_keys.push_str(" )");
                insert_values.push_str(" )");
            }
            else {
                insert_keys.push_str(&format!("{}, ", params[i].0)); //key at first point in tuple
                insert_values.push_str(&format!("'{}', ", params[i].1)); // value in second
            }
        }

        //combine query pieces
        insert_keys.push_str(&insert_values);
        table_statement.push_str(&insert_keys);

        println!("TABLE STATEMENT: {:?}", table_statement.clone());
        table_statement

}


pub fn write_to_table(
    sql: String,
    pool: Pool,) -> () {

        let mut conn = pool.get_conn().unwrap();

        conn.query(sql).map(|query_result| {
                 println!("Query Results: {:?}", query_result.last_insert_id());
        }); // Currently, no error handling or confirmation of return
        // should make custom struct with all return details as norm
}

  //*****************************************************/
 //**************** Combined Functions *****************/
//*****************************************************/

pub fn basic_write_to_table(
    table: String,
    params: Vec<(String, String)>,
    pool: Pool) -> () {

        let sql: String = simple_insert_statement(table, params);
        write_to_table(sql, pool);
}

pub fn get_account_json_by_id(
    identifier: &str,
    table: &str,
    pool: Pool,) -> Vec<String> {

        get_table_json_by_param("id", identifier, table, pool)
}
