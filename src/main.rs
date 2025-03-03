// mod args;
// pub mod storing;
// pub mod searching;
// pub mod apis;
// mod err;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello lib!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

