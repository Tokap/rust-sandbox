impl User {
    pub fn get_field_names() -> Vec<String> {
        let mut field_names: Vec<String> = Vec::new();
        let temp = json!(User::default());
        let er =
            if let serde_json::Value::Object(o) = temp { o }
            else {unreachable!()};
        for (key, _) in er {
            field_names.push(key);
        }
        field_names
    }
}
