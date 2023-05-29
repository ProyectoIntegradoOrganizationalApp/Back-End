use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/task_app/board", data="<board_info>", format="json")]
pub fn create_board(board_info: Validated<Json<BoardInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Board>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::create_board(&board_info.0, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/task_app/board/<id>", data="<board_info>", format="json")]
pub fn update_board(id: String, board_info: Validated<Json<BoardInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::task_app::board::update_board(&board_info.0, &id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/task_app/board/<id>")]
pub fn delete_board(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::task_app::board::delete_board(&id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}