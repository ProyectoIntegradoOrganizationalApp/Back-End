use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/task_app/column", data="<column_info>", format="json")]
pub fn create_column(column_info: Validated<Json<ColumnInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Columna>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::column::create_column(&column_info.0, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/task_app/column/<id>", data="<column_info>", format="json")]
pub fn update_column(id: String, column_info: Validated<Json<ColumnInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::task_app::column::update_column(&column_info.0, &id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/task_app/column/<id>")]
pub fn delete_column(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::task_app::column::delete_column(&id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}