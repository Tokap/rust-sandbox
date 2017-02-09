use serde_json;
use serde_json::Value;

pub fn return_truncated_json(v: Value, i: usize) -> String {
    let vec;
    match v.as_array() {
        Some(r) => vec = r,
        None => return format!("Cannot Convert to vector!")
    }

    let my_slice = &vec[0..i];

    match serde_json::to_string(&my_slice) {
        Ok(r) => r,
        Err(_) => format!("Nothing to Return")
    }
}
