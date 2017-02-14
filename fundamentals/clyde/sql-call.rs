let mut conn = my::Conn::new("mysql://root@localhost/ip_dashboard").unwrap();
let sql = "
	select
		id,
		username,
		name_first,
		name_last
	from user
	limit 5";

let temp = conn.query(sql);

let lj: Vec<String> = User::get_field_names();

// collect all query result values as a string
// rows separated by '|||'
let bob: Vec<_> = temp.map(|query_result| {
	query_result.map(|result_row| result_row.unwrap()).map(|row| {
		let value_vec = row.unwrap();
		let mut stringg: String = String::new();
		for value in value_vec {
			let stringy: String = my::from_value(value);
			stringg.push_str(stringy.as_str());
			stringg.push_str("|||");
		};
		stringg
	}).collect()
}).unwrap();

// turn values string into a JSON string
// rows separated by '|||'
let mut json: String = String::new();
for i in bob {
	let mut vec_i: Vec<&str> = i.split("|||").collect();
	vec_i.pop();
	json.push_str(" { ");
	for (index, val) in vec_i.into_iter().enumerate() {
		let field_index: &str = lj[index].as_str();
		json.push_str("\"");
		json.push_str(field_index);
		json.push_str("\": \"");
		json.push_str(val);
		json.push_str("\", ");
	}
	let len = json.len();
	json.truncate(len - 2);
	json.push_str(" } |||")
}

/// Convert single JSON string into a vector of JSON strings
// pop added to remove empty entry
let mut vec_user: Vec<User> = Vec::new();
let mut vec_json: Vec<&str> = json.split("|||").collect();
vec_json.pop();


// Convert JSON string vector into vector of defined type
// this might be one that stays with type impl????
for b in vec_json {
	let te: User = serde_json::from_str(b).unwrap();
	vec_user.push(te);
}
for user in vec_user {
	println!("{:?}", user);
}
