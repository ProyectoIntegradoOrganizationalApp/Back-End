use rocket::serde::json::{Json};
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