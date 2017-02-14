// Converting a String into a custom object:
let test_deuxe = r#" { "network": "twitter Doe", "network_id": "123456", "network_username": "funky_trunks",
                "follower_count": "12345", "correlation_id": "X12Y3456", "archive_id": "2", "type_id": "2" } "#;

let v = serde_json::from_str::<AccountDataWrite>(test_deuxe).unwrap();
