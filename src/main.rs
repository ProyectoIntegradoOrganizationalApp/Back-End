

use tokio::net::TcpListener;
#[allow(unused)]
use rocket::{serde::{json::{json, Value}}, tokio::{signal::ctrl_c, self}};
use routes::auth::{register, login, send_mail, change_password, logout, test_token, update_user, delete_user};
use routes::user::{achievements, user_achievements, profile, send_friend_request, accept_friend_request, deny_friend_request, delete_user_friend,
                    user_notifications, user_friends, search_users, account};
use routes::project::{create_project, update_project, delete_project, invite_user_to_project, change_role_user_project, delete_user_project, 
    accept_user_project_invitation, deny_user_project_invitation, get_project, get_user_projects, leave_project, search_projects};
use routes::task_app::board::{create_board, update_board, delete_board, get_boards};
use routes::task_app::column::{create_column, update_column, delete_column, get_columns_tasks};
use routes::task_app::task::{create_task, update_task, delete_task};
use routes::app::{create_app, update_app, delete_app};
use rocket_sync_db_pools::database;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use dotenvy::dotenv;

use services::chat::websocket_handler;

// use crate::services::chat::ws_test;

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

#[rocket::main]
async fn main() {
    dotenv().ok();
    // Iniciamos servidor ws
    let listener = TcpListener::bind("127.0.0.1:9001").await.expect("Failed to bind");

    let rocket = rocket::build()
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
            user_notifications,
            user_friends,
            profile,
            account,
            update_user,
            delete_user,
            create_project,
            update_project,
            delete_project,
            create_app,
            update_app,
            delete_app,
            create_board,
            update_board,
            delete_board,
            get_boards,
            get_columns_tasks,
            create_column,
            update_column,
            delete_column,
            create_task,
            update_task,
            delete_task,
            get_project,
            get_user_projects,
            invite_user_to_project,
            accept_user_project_invitation,
            deny_user_project_invitation,
            change_role_user_project,
            delete_user_project,
            leave_project,
            send_friend_request,
            accept_friend_request,
            deny_friend_request,
            delete_user_friend,
            search_users,
            search_projects
        ])
        .register("/", catchers![not_found, server_error, rocket_validation::validation_catcher]);


        // Thread listener de servidor ws
        tokio::spawn(async move {
            loop {
                let socket = listener.accept().await.unwrap();
                tokio::spawn(websocket_handler(socket.0));
            }
        });
    
        // ws_server.await.expect("WebSocket server failed");
        
        rocket.launch().await.expect("ERROR ROCKET");
}