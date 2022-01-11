#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is a test! I'll work on getting an actual frontend set up once I figure out how to deliver HTML pages..."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}