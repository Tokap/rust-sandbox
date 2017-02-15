/*******************************************************/
/************* Manual JSON Creation - Ugly ************/
/*****************************************************/
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
/******** Attempt at nested maps & conversions ********/
/*****************************************************/

// #[allow(dead_code)]
// pub fn get_by_param_with_handling(
//     param: &str,
//     identifier: &str,
//     table: &str,
//     pool: Pool,) -> String {
//         let mut conn = pool.get_conn().unwrap();
//
//         let mut all_row_values: Vec<Vec<String>> = Vec::new();
//         let mut return_array = json::JsonValue::new_array();
//
//         let sql = format!("SELECT * FROM `{}` WHERE `{}`={}", table, param, identifier);
//
//         // let col_name_vec: Vec<String> = get_col_names(&query_result);
//         // : Result<String, String>
//         let ending_result =
//         conn.query(sql)
//         .map_err(|e| e.to_string())
//         .map(|mut query_result| {
//             // println!("My thing: {:?}", query_result);
//             query_result.map(|x| x.unwrap()).map(|row| {
//                 let x: String = from_row(row.clone());
//                 println!("Moded X Var: {:?}", x);
//
//                 // println!("From Row: {:?}", x);
//                 row
//                 // row.unwrap().unwrap().into_iter().map(|unwrapped| {
//                 //     println!("My thing: {:?}", unwrapped.into_str());
//                 //     // println!("Col Names: {:?}", col_name_vec);
//                 //     unwrapped.into_str();
//                 // })
//             })
//
//         });
//
//         println!("Moded Conn Var: {:?}", ending_result);
//
//         "return_array".to_string()
// }
