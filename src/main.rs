#[macro_use] extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::serde::{json::*};

#[get("/")]
fn api() -> Value {
    json!({
        "test": true
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("/frontend/public")))
        .mount("/api", routes![api])
}