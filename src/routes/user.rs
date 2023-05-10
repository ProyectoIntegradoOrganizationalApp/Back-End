use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;
use crate::utilities::achievements::*;

// #[get("/profile/<id>")]
// pub fn profile(id: String) -> Result<Json<User>, Json<GenericError>> {
//     Json(id)
// }































// Siguiente endpoint
#[get("/achievements")]
pub fn achievements(token: Result<TokenValidantion, GenericError>) -> Result<Json<AllAchievementsResponse>, Json<GenericError>> {
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
pub fn user_achievements(token: Result<TokenValidantion, GenericError>, id: String) -> Result<Json<UserAchievementsResponse>, Json<GenericError>> {
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