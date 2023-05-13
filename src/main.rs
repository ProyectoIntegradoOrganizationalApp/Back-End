use rocket::{serde::{json::{json, Value}}};
use routes::auth::{register, login, send_mail, change_password, logout, test_token, update_user};
use routes::user::{achievements, user_achievements, profile};
use rocket_sync_db_pools::database;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use dotenvy::dotenv;

#[macro_use] extern crate rocket;

mod routes;
mod services;
mod models;
mod schema;
mod utilities;
mod middlewares;

#[database("my_db")]
pub struct Db(diesel::PgConnection);
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

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
        .attach(Cors)
        .attach(Db::fairing())
        .mount("/", routes![
            register, 
            login, 
            send_mail, 
            change_password, 
            logout, 
            all_options, 
            test_token, 
            achievements, 
            user_achievements, 
            profile,
            update_user
        ])
        .register("/", catchers![not_found, server_error])
}
