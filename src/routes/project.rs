use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/project", data="<project_info>", format="json")]
pub fn create_project(project_info: Json<ProjectInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<Project>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::create_project(&project_info, token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}