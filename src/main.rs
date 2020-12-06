#![feature(decl_macro, proc_macro_hygiene)]

#[cfg(test)] mod tests;

use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
