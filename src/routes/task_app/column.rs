use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/<id_app>/task_app/column", data="<column_info>", format="json")]
pub fn create_column(id_app: String, column_info: Validated<Json<ColumnInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Columna>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::column::create_column(&column_info.0, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/<id_app>/task_app/column/<id>", data="<column_info>", format="json")]
pub fn update_column(id_app: String, id: String, column_info: Validated<Json<ColumnInputUpdate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::column::update_column(&column_info.0, &id, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/<id_app>/task_app/column/<id>")]
pub fn delete_column(id_app: String, id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::column::delete_column(&id, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/<id_app>/task_app/<id_board>/columns")]
pub fn get_columns_tasks(id_app: String, id_board: String, token: Result<TokenValidation, GenericError>) -> Result<Json<Vec<ColumnTasks>>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::column::get_columns_tasks(&id_app, &id_board, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}