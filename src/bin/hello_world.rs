#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::Json;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::response::Failure;
use rocket::http::Status;

#[get("/")]
fn hello() -> &'static str {
    "Hello World!"
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[get("/person/<name>/<age>")]
fn person(name: String, age: u8) -> Json<Person> {
    Json(Person { name, age })
}

#[get("/drink/<kind>")]
fn drink(kind: String) -> Result<&'static str, Failure> {
    if kind == "coffee" {
        Err(Failure(Status::ImATeapot))
    } else {
        Ok("Here's your drink")
    }
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/secret")]
fn redir() -> Redirect {
    Redirect::temporary("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, person, drink, files, redir])
        .launch();
}
