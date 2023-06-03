use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/<id_app>/task_app/task", data="<task_info>", format="json")]
pub fn create_task(id_app: String, task_info: Validated<Json<TaskInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Task>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::task::create_task(&task_info.0, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/<id_app>/task_app/task/<id>", data="<task_info>", format="json")]
pub fn update_task(id_app: String, id: String, task_info: Validated<Json<TaskInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::task::update_task(&task_info.0, &id, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/<id_app>/task_app/task/<id>")]
pub fn delete_task(id_app: String, id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::task_app::task::delete_task(&id, &id_app, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}