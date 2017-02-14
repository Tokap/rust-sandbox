// Originally in an experimental main file
let x = AccountDataWrite::default();

let trial_run: AccountDataWrite = AccountDataWrite {
    network: "twitter".to_string(),
    network_id: "8675309".to_string(),
    network_username: "imgenerallyawesome".to_string(),
    follower_count: "1234567".to_string(),
    correlation_id: "12XD56HY34".to_string(),
    archive_id: "1".to_string(),
    type_id: "2".to_string(),
};

let dos: AccountDataWrite = trial_run.clone();
let tres: AccountDataWrite = trial_run.clone();

let qq = serde_json::to_string(&trial_run);

let json_string = json!({
    "network": "twitter",
    "network_id": "123456",
    "network_username": "funky_trunks",
    "follower_count": "12345",
    "correlation_id": "X12Y3456",
    "archive_id": "2",
    "type_id": "2",
});


let pool: Pool = build_pool("127.0.0.1", "ip_brolytics", "root", 3306);
get_account_data_by_id("1", pool.clone());
// write_account_data(pool.clone(), trial_run);

println!("AccountData Review: {:?}", trial_run.network);
println!("Default Serialize: {:?}", qq);
// server::start_server();

let john = json!({
  "name": "John Doe",
  "age": 43,
});

let j = serde_json::to_string(&john).unwrap();

println!("Finally Encoded!!: {:?}", j );
println!("Check Fields!!!: {:?}", dos.get_field_names());


let json_string = json!({
    "network": "twitter",
    "network_id": "123456",
    "network_username": "funky_trunks",
    "follower_count": "12345",
    "correlation_id": "X12Y3456",
    "archive_id": "2",
    "type_id": "2",
});

let test_deuxe = r#" { "network": "twitter Doe", "network_id": "123456", "network_username": "funky_trunks",
                "follower_count": "12345", "correlation_id": "X12Y3456", "archive_id": "2", "type_id": "2" } "#;

let v = serde_json::from_str::<AccountDataWrite>(test_deuxe).unwrap();
