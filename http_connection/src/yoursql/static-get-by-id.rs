/*******************************************************/
/* Original get_data_by_param with static type coding */
/*****************************************************/

pub fn get_account_data_by_param(param: &str, identifier: &str, pool: Pool) -> Vec<AccountDataArchive> {

    let mut conn = pool.get_conn().unwrap(); // Get a connection to the pool

    let mut all_returns: Vec<Vec<String>> = Vec::new();
    let mut account_data_vec: Vec<AccountDataArchive> = Vec::new();
    let params = format!("SELECT * FROM `account_data_archive` WHERE `{}`={}", param, identifier);

    conn.query(params).map(|query_result| {

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
