#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::env;

const FAKE_CONTENT: &str = "Hello, Facebook bot!";
const REAL_CONTENT: &str = "Hello, regular person!";


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
