/*** Some notes about using the Json library ***/
let stringify = json::stringify("foobar cat".to_string()); // encapsulates in quotes
assert_eq!(json::stringify(json::Null), "null"); // use of null
let data = vec![1,2,3];
assert_eq!(json::stringify(data), "[1,2,3]");

// Adding Data to Objects
let mut data = json::JsonValue::new_object();
data["answer"] = 42.into();
data["foo"] = "bar".into();
assert_eq!(data.dump(), r#"{"answer":42,"foo":"bar"}"#);

// data.entries() will return key, value pairs
// data.members() will return each index of an array

// My Stuff:
let json_string = json::parse(r#"
{
    "name": "john",
    "age": 25,
    "data": {
        "debt": 10000.00,
        "income": 400.00
    }
}
"#).unwrap();

let instantiated = object!{
"code" => 200,
"success" => true,
"payload" => object!{
    "features" => array![
        "awesome",
        "easyAPI",
        "lowLearningCurve"
        ]
    }
};

// Make an array:
let mut data = json::JsonValue::new_array();
data.push(10);
data.push("foo");
data.push(false);
assert_eq!(data.dump(), r#"[10,"foo",false]"#);
