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

// Result<PooledConn, Error>

fn test_and_output_connection(p: Pool) -> () {
    match p.try_get_conn(50000) {
        Ok(rez) => println!("Okay! {:?}", rez),
        Err(e) => println!("Bad news: {:?}", e),
    }
}

/* Simplified and without notes: */

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
    /***********************************************************************************/
    /***************** A Very Basic MySQL Query & Explanations  ***********************/
    /*********************************************************************************/

    // The building blocks we need:
    let z = build_pool(); // Our pool. For this ex, build from static info shown above
    let mut conn = z.get_conn().unwrap(); // Get a connection to the pool


    let mut container: Vec<String> = Vec::new(); //Container for string building later
    let mut all_returns: Vec<Vec<String>> = Vec::new();
    let mut return_vec: Vec<AccountDataArchive> = Vec::new();

    // Making the query in painful detail:
    // NOTE: all stages of the query use custom Structs for the rust-mysql library
    // For more info on manipulation options at each stage, see the rust-mysql docs

    conn.query("SELECT * FROM `account_data_archive`").map(|mut query_result| {
        // query_result is a mysql::QueryResult that contains a Row for each piece of matching data
        // QueryResults has a number of  associated methods as a result of being an iterator
        // This includes search, map and filter features


        // column_indexes() will return return a hashmap of the column names & their index
        // This list is usually out of order, so you should sort it if going that way
        // Return type is: HashMap<String, usize, BldHshrDflt<FnvHasher>>
        // To iterate: for (col_name, index) in &hash_map {}


        // Get Column Names & sort for future mapping
        let column_hash_map = query_result.column_indexes();
        let mut col_tuple_vec: Vec<(String, usize)> = Vec::new();
        for (name, idx) in column_hash_map.iter() {
            col_tuple_vec.push( (name.to_string(), *idx) )
        }
        col_tuple_vec.sort_by(|a,b| a.1.cmp(&b.1));

        println!("Column Map: {:?}", column_hash_map);
        println!("Column Length: {:?}", column_hash_map.len());

         for row in query_result {
            // ROW before anything is a Result<mysql::Row, mysql::Error>
            // unwrap once for Row & again to unwrap the Values of the Row -> Vec<Value>

            // Apparently this is the right way to get row data:
            // mysql::value::from_row(row);

            // I clone rows for different uses:
            let mut row_clone = row.unwrap().clone();
            let row_clone_two = row_clone.clone();
            let unwrapped = row_clone_two.unwrap();

            let p_unwrapp = unwrapped.clone();
            let mut row_returns: Vec<String> = Vec::new(); // My return box

            for value in p_unwrapp {
                    row_returns.push(value.into_str())
            }

            all_returns.push(row_returns);

            /************************* Using Values in Rows ************************/
            // iterate through unwrapped values:
            let f_unwrapped = unwrapped.clone();
            for value in f_unwrapped {
                container.push(value.into_str()); // push column data into vector
                container.push(" | ".to_string()); // push divider for readability
            }
            // combine container into string and print result:
            let row_content_string: String = container.join("");
            // println!("Summarized Row Content: {}", row_content_string);
            /**********************************************************************/

            // take allows us to grab specific column data in a row. This can obv fail:
            let take: String = row_clone.take("network_username").unwrap();


            let length = unwrapped.clone().len(); // = # of columns = 12 in this ex.

            println!("Row Take: (Username): {:?}", take);
            for value in unwrapped.clone() {
                // If handled as suggested above, we're now looking at a mysql::Value
                // I could theoretically make objects out of the rows if done right.
                println!("Here are some query results {}", value.into_str());
            }


        }

        /****************** Creating Vec<AccountDataArchive> from returns ********************/
        let mut account_data_vec: Vec<AccountDataArchive> = Vec::new();
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
    });

    // println!("RESULT: {:?}", all_returns);
    // server::start_server();
}
