// ROUTE BASICS

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "Hello, world!"
}

#[get("/user/<id>")]
fn user(id: usize) -> T { ... }

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> T { ... }

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> T { ... }

// You can also match against multiple segments by using <param..> in the route path.
#[get("/page/<path..>")]
fn get_page(path: PathBuf) -> T { ... }

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// Handling incoming data from a form:
#[derive(FromForm)]
struct Task {
    complete: bool,
    description: String,
}

#[post("/todo", data = "<task>")]
fn new(task: Form<Task>) -> String { ... }

// Handling JSON
#[derive(Deserialize)]
struct Task {
    description: String,
    complete: bool
}

#[post("/todo", data = "<task>")]
fn new(task: JSON<Task>) -> String { ... }

// Good Rocket Example of Dealing with JSON & Pathing:
// https://github.com/SergioBenitez/Rocket/blob/master/examples/json/src/main.rs

// WRITING UNIT TESTS IN RUST:
// http://stackoverflow.com/questions/25107526/how-should-rust-unit-tests-be-organised

// And_thens && Unwrap Error Handling

// std::fs::File::open("Settings.toml")
//      .and_then(|file| file.read_to_string(&mut data))
//      .and_then(|size| serde_toml::from_str(data))
//      .unwrap_or(Err("Invalid Settings.toml"))

/*****************************************/
/******* Nate's Call Function ***********/
/***************************************/

// Call function using expected Url Type
// fn call(url: Url) -> Outcome {
//     client()
//         .get(url)
//         // .body(body)
//         .send()
//         .map_err(|x| format!("{:?}", x))
//         .map(|x| {
//             print!("RES: {:?}", &x);
//             x
//         })
//         .and_then(|r| serde_json::from_reader(r).map_err(|x| format!("{:?}", x)))
// }

// let builder = UrlBuilder::new(root, slugger);
// let compiled = builder.compile();
// let newer = call(compiled).tester.unwrap().to_owned();
