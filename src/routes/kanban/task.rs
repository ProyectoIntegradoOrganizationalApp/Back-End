use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/task", data="<task_info>", format="json")]
pub fn create_task(task_info: Validated<Json<TaskInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Task>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::kanban::task::create_task(&task_info.0, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/task/<id>", data="<task_info>", format="json")]
pub fn update_task(id: String, task_info: Validated<Json<TaskInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::task::update_task(&task_info.0, &id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/task/<id>")]
pub fn delete_task(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match services::kanban::task::delete_task(&id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}