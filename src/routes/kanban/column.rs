use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/column", data="<column_info>", format="json")]
pub fn create_column(column_info: Json<ColumnInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<Columna>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::column::create_column(&column_info) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/column/<id>", data="<column_info>", format="json")]
pub fn update_column(id: String, column_info: Json<ColumnInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::column::update_column(&column_info, &id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/column/<id>")]
pub fn delete_column(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::column::delete_column(&id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}