use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/project/<project_id>/app", data="<app_info>", format="json")]
pub fn create_app(project_id: String, app_info: Json<AppInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<App>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::app::create_app(&app_info, &project_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/project/<project_id>/app/<app_id>", data="<app_info>", format="json")]
pub fn update_app(project_id: String, app_id: String, app_info: Json<AppInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::app::update_app(&app_info, &token_data.token_iduser, &project_id, &app_id) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/project/<project_id>/app/<app_id>")]
pub fn delete_app(project_id: String, app_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::app::delete_app(&app_id, &project_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}