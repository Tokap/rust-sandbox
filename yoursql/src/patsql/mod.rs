#![feature(use_extern_macros)]

use mysql::{OptsBuilder, Pool, QueryResult}; // PooledConn, Error, Value,
use json;

/*******************************************************/
/************** Creating a Pool Connection ************/
/*****************************************************/

// expand to allow for more options
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn test_and_output_connection(p: Pool) -> bool {

    match p.try_get_conn(50000) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/*******************************************************/
/***************** Reading from mysql *****************/
/*****************************************************/

// SUPPORT FN:
#[allow(dead_code)]
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

#[allow(dead_code)]
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
/***************** NEW READ FUNCTION  *****************/
/*****************************************************/

#[allow(dead_code)]
pub fn get_by_param(
    param: &str,
    identifier: &str,
    table: &str,
    pool: Pool,) -> String {

        let mut conn = pool.get_conn().unwrap();

        let mut return_array = json::JsonValue::new_array();
        let mut all_row_values: Vec<Vec<String>> = Vec::new();

        let params = format!("SELECT * FROM `{}` WHERE `{}`={}", table, param, identifier);

        conn.query(params).map(|query_result| {
            let col_name_vec: Vec<String> = get_col_names(&query_result);

            // Create Vector of Vec<String> holding value on each row w/o keys
            for row in query_result {
                let unwrapped = row.unwrap().unwrap();
                let mut row_returns: Vec<String> = Vec::new();

                for value in unwrapped {
                    row_returns.push(value.into_str())
                }

                all_row_values.push(row_returns);
            }

            // Go through each row's content, assign it a key with col names & create JSON
            for row_contents in all_row_values {

                let mut data_object = json::JsonValue::new_object();
                let col_names: Vec<String> = col_name_vec.clone();

                for i in 0..row_contents.len() {
                    data_object[col_names[i].to_owned()] = row_contents[i].replace("'", "").into();
                }

                return_array.push(data_object);
            }
        });

        return_array.dump()
}

/*******************************************************/
/************* Manual JSON Creation - Ugly ************/
/*****************************************************/

// @TODO: Should make a more tailored version that takes json
#[allow(dead_code)]
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

        table_statement

}

/*******************************************************/
/***************** NEW WRITE FUNCTION  ****************/
/*****************************************************/
#[allow(dead_code)]
pub fn simple_json_insert(
    table: String,
    json: String ) -> String {

        let table_statement: String = format!("INSERT INTO `{}`", table);
        let mut key_vec: Vec<String> = Vec::new();
        let mut value_vec: Vec<String> = Vec::new();


        let json_obj = json::parse(&json).unwrap();
        let keys_and_values: json::object::Iter = json_obj.entries();

        for i in keys_and_values {
            println!("Key/Value Pairs: {:?}", i);
            println!("Value: {:?}", i.1);
            key_vec.push(i.0.to_string());
            value_vec.push(format!("'{}'",i.1));
        }

        let keys: String = format!("({})", key_vec.join(", "));
        let values: String = format!(" VALUES ({})", value_vec.join(", "));

        [table_statement, keys, values].join(" ") // Return combined statement

}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SqlWriteReturn {
    last_save_id: u64,
    affected_rows: u64,
    warning_count: u16,
}

#[allow(dead_code)]
pub fn write_to_table(
    sql: String,
    pool: Pool,) -> Result<SqlWriteReturn, String> {

        let mut conn = pool.get_conn().unwrap();

        let final_return: Result<SqlWriteReturn, String> = conn.query(sql)
        .map_err(|e| e.to_string() )
        .map(|query_result| {
             SqlWriteReturn {
                 last_save_id: query_result.last_insert_id(),
                 affected_rows: query_result.affected_rows(),
                 warning_count: query_result.warnings(),
             }
        });

        final_return
}

  //*****************************************************/
 //**************** Combined Functions *****************/
//*****************************************************/
#[allow(dead_code)]
pub fn vec_write_to_table(
    table: String,
    params: Vec<(String, String)>,
    pool: Pool) -> Result<SqlWriteReturn, String> {

        let sql: String = simple_insert_statement(table, params);
        write_to_table(sql, pool)
}

#[allow(dead_code)]
pub fn json_write_to_table(
    table: String,
    params: String,
    pool: Pool) -> Result<SqlWriteReturn, String> {

        let sql: String = simple_json_insert(table, params);
        write_to_table(sql, pool)
}

#[allow(dead_code)]
pub fn get_json_by_id(
    identifier: &str,
    table: &str,
    pool: Pool,) -> Vec<String> {

        get_table_json_by_param("id", identifier, table, pool)
}
