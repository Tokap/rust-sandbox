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
