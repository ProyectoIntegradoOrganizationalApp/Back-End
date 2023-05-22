use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;
use crate::utilities::achievements::*;

#[get("/achievements")]
pub fn achievements(token: Result<TokenValidation, GenericError>) -> Result<Json<AllAchievementsResponse>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match get_all_achievements() {
                Ok(achievements) => {
                    let response = AllAchievementsResponse {
                        total: achievements.len(),
                        achievements
                    };
                    Ok(Json(response))
                },
                Err(e) => Err(Json(e))
            }
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}

#[get("/profile/<id>/achievements")]
pub fn user_achievements(token: Result<TokenValidation, GenericError>, id: String) -> Result<Json<UserAchievementsResponse>, Json<GenericError>> {
    match token {
        Ok(_) => {
            match get_user_achievements(&id) {
                Ok(achievements) => {
                    let response = UserAchievementsResponse {
                        total: achievements.len(),
                        achievements
                    };
                    Ok(Json(response))
                },
                Err(e) => Err(Json(e))
            }
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}

#[get("/profile/<id>")]
pub fn profile(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<UserProfile>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::profile(&id) {
                Ok(mut result) => {
                    result.owner = token_data.owner;
                    Ok(Json(result))
                },
                Err(err) => {
                    Err(Json(GenericError {
                        error: true,
                        message: err
                    }))
                }
            }
        },
        Err(err) => Err(Json(err))
    }
}

#[post("/user/<user_id>/project/<project_id>", data="<invitation>", format="json")]
pub fn invite_user_to_project(user_id: String, project_id: String, invitation: Validated<Json<InvitationMessage>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::invite_user_to_project(&user_id, &project_id, &token_data.token_iduser, &invitation.0) {
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
            match services::user::change_role_user_project(&user_id, &project_id, &token_data.token_iduser, &role) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}