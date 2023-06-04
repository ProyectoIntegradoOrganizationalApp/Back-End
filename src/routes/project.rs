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

#[post("/user/<user_id>/project/<project_id>", data="<invitation>", format="json")]
pub fn invite_user_to_project(user_id: String, project_id: String, invitation: Validated<Json<InvitationMessage>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::invite_user_to_project(&user_id, &project_id, &token_data.token_iduser, &invitation.0) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/user/<user_id>/project/<project_id>/accept")]
pub fn accept_user_project_invitation(project_id: String, user_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::accept_user_project_invitation(&project_id, &user_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/user/<user_id>/project/<project_id>/deny")]
pub fn deny_user_project_invitation(project_id: String, user_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::deny_user_project_invitation(&project_id, &user_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[put("/user/<user_id>/project/<project_id>", data="<role>", format="json")]
pub fn change_role_user_project(user_id: String, project_id: String, role: Json<NewRole>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::change_role_user_project(&user_id, &project_id, &token_data.token_iduser, &role) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[delete("/user/<user_id>/project/<project_id>")]
pub fn delete_user_project(user_id: String, project_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::delete_user_project(&user_id, &project_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[delete("/project/<project_id>/leave")]
pub fn leave_project(project_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::project::leave_project(&project_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
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
            match services::project::get_user_projects(&id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => Err(Json(err))
    }
}