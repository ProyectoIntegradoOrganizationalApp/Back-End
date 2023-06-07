use rocket::{serde::json::{Json}, futures::{StreamExt, SinkExt}};
use crate::models::models::*;
use crate::services;

#[post("/message", data="<message_info>", format="json")]
pub fn create_message(message_info: Json<MessageInput>, token: Result<TokenValidation, GenericError>) -> Result<Json<Message>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::create_message(&token_data.token_iduser, &message_info) {
                Ok(message) => Ok(Json(message)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/message/<id>", data="<message_info>", format="json")]
pub fn update_message(id: String, message_info: Json<MessageUpdate>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::update_message(&id, &token_data.token_iduser, &message_info) {
                Ok(message) => Ok(Json(message)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/message/<id>")]
pub fn delete_message(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::delete_message(&id, &token_data.token_iduser) {
                Ok(message) => Ok(Json(message)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/websocket")]
pub fn websocket_route() -> &'static str {
    "Conexión WebSocket establecida"
}