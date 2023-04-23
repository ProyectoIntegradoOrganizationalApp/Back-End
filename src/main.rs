#[macro_use] extern crate rocket;

mod routes;
mod services;
mod models;
mod schema;
mod utilities;
mod middlewares;

use rocket::{serde::{json::{json, Value}}};
use routes::auth::{register, login, send_mail, change_password, test_token};
use rocket_sync_db_pools::database;
use dotenvy::dotenv;

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "Error",
        "reason": "Resource was not found"
    })
}

#[catch(500)]
fn server_error() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again later"
    })
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount("/", routes![register, login, send_mail, change_password, test_token])
        .register("/", catchers![not_found, server_error])
}
