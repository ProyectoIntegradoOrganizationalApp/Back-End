use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/board", data="<board_info>", format="json")]
pub fn create_board(board_info: Json<BoardInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<Board>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::board::create_board(&board_info) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/board/<id>", data="<board_info>", format="json")]
pub fn update_board(id: String, board_info: Json<BoardInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::board::update_board(&board_info, &id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/board/<id>")]
pub fn delete_board(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::board::delete_board(&id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}