use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/<id_app>/task_app/board", data="<board_info>", format="json")]
pub fn create_board(id_app: String, board_info: Validated<Json<BoardInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Board>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::create_board(&id_app, &board_info.0, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/<id_app>/task_app/board/<id>", data="<board_info>", format="json")]
pub fn update_board(id_app: String, id: String, board_info: Validated<Json<BoardInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::update_board(&id_app, &board_info.0, &id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/<id_app>/task_app/board/<id>")]
pub fn delete_board(id_app: String, id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::delete_board(&id_app, &id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/<id_app>/task_app/boards")]
pub fn get_boards(id_app: String, token: Result<TokenValidation, GenericError>) -> Result<Json<Vec<Board>>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::get_boards(&id_app, &token_data.token_iduser) {
                Ok(boards) => Ok(Json(boards)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/board/<board_id>")]
pub fn get_board(board_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<Board>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::board::get_board(&board_id, &token_data.token_iduser) {
                Ok(boards) => Ok(Json(boards)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}