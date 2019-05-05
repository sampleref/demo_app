#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn index(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", name, age)
}

fn main() {
    rocket::ignite().mount("/hello", routes![index]).launch();
}