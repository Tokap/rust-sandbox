use mysql::{Pool};
use json;

/*******************************************************/
/*********** Return Struct for Writing  ***************/
/*****************************************************/

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SqlWriteReturn {
    last_save_id: u64,
    affected_rows: u64,
    warning_count: u16,
}

/*******************************************************/
/**************** Core Write Functions ****************/
/*****************************************************/

#[allow(dead_code)]
pub fn simple_json_insert(
    table: String,
    json: String ) -> Result<String, String> {

        let table_statement: String = format!("INSERT INTO `{}`", table);
        let mut key_vec: Vec<String> = Vec::new();
        let mut value_vec: Vec<String> = Vec::new();

        let mut obj: json::JsonValue = json::JsonValue::new_object();


        let json_obj = json::parse(&json);

        if json_obj.is_err() {
            println!("Error: Could not convert String into JSON");
            return Err( String::from("Could not convert into JSON"));
        }
        else {
            obj = json_obj.unwrap();

            let keys_and_values: json::object::Iter = obj.entries();

            for i in keys_and_values {
                key_vec.push(i.0.to_string());
                value_vec.push(format!("'{}'",i.1));
            }

            let keys: String = format!("({})", key_vec.join(", "));
            let values: String = format!(" VALUES ({})", value_vec.join(", "));

            Ok([table_statement, keys, values].join(" ")) // Return combined statement
    }

}

#[allow(dead_code)]
pub fn simple_vec_insert(
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
                insert_keys.push_str(&format!("{}, ", params[i].0));
                insert_values.push_str(&format!("'{}', ", params[i].1));
            }
        }

        //combine query pieces
        insert_keys.push_str(&insert_values);
        table_statement.push_str(&insert_keys);

        table_statement

}

#[allow(dead_code)]
pub fn write_to_table(
    sql: String,
    pool: Pool,) -> Result<SqlWriteReturn, String> {
        println!("Passed Beginning!");

        let mut conn = pool.clone().try_get_conn(500);

        println!(" Conn is err: {:?}", conn.is_err());

        if conn.is_err() { Err(String::from("Connection details are invalid.")) }
        else {
            println!("Passed Validation!");
            let mut validated_connection = pool.get_conn().unwrap();
            let final_return: Result<SqlWriteReturn, String> = validated_connection.query(sql)
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
}

  //*****************************************************/
 //************* Combined Write Functions **************/
//*****************************************************/

#[allow(dead_code)]
pub fn vec_write_to_table(
    params: Vec<(String, String)>,
    table: String,
    pool: Pool) -> Result<SqlWriteReturn, String> {

        let sql: String = simple_vec_insert(table, params);
        write_to_table(sql, pool)
}

#[allow(dead_code)]
pub fn json_write_to_table(
    params: String,
    table: String,
    pool: Pool) -> Result<SqlWriteReturn, String> {

        let sql: String = simple_json_insert(table, params).unwrap();
        write_to_table(sql, pool)
}

#[allow(dead_code)]
pub fn raw_write_to_table(
    sql: String,
    pool: Pool) -> Result<SqlWriteReturn, String> {

        write_to_table(sql, pool)
}
