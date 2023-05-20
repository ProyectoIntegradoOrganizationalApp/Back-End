use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;

#[post("/project", data="<project_info>", format="json")]
pub fn create_project(project_info: Validated<Json<ProjectInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<Project>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::create_project(&project_info.0, token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[put("/project/<id>", data="<project_info>", format="json")]
pub fn update_project(id: String, project_info: Validated<Json<ProjectInputCreate>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            if token_data.owner {
                match services::project::update_project(&project_info.0, &token_data.token_iduser, &id) {
                    Ok(result) => Ok(Json(result)),
                    Err(err) => Err(Json(err))
                }
            } else {
                Err(Json(GenericError { error: true, message: "You are not the owner".to_string()}))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[delete("/project/<id>")]
pub fn delete_project(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            if token_data.owner {
                match services::project::delete_project(&token_data.token_iduser, &id) {
                    Ok(result) => Ok(Json(result)),
                    Err(err) => Err(Json(err))
                }
            } else {
                Err(Json(GenericError { error: true, message: "You are not the owner".to_string()}))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/project/<id>")]
pub fn get_project(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<ProjectDetail>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::get_project(&id, &token_data) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[get("/user/<id>/projects")]
pub fn get_user_projects(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<UserProjects>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            if token_data.owner {
                match services::project::get_user_projects(&id) {
                    Ok(result) => Ok(Json(result)),
                    Err(err) => Err(Json(err))
                }
            } else {
                Err(Json(GenericError { error: true, message: "You are not the owner".to_string()}))
            }
        },
        Err(err) => Err(Json(err))
    }
}