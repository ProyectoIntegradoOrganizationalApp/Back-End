use rocket::serde::json::{Json};
use rocket_validation::Validated;
use crate::models::models::*;
use crate::services;
use crate::utilities::achievements::*;
use crate::utilities::notifications::*;

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
            match services::user::get_user_achievements(id) {
                Ok(achievements) => Ok(Json(achievements)),
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

#[post("/friend/<guest_id>", data="<invitation>", format="json")]
pub fn send_friend_request(guest_id: String, invitation: Validated<Json<InvitationMessage>>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::send_friend_request(&guest_id, &invitation.0, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/friend/<user_id>/accept")]
pub fn accept_friend_request(user_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::accept_friend_request(&user_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/friend/<user_id>/deny")]
pub fn deny_friend_request(user_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::deny_friend_request(&user_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[delete("/friend/<friend_id>")]
pub fn delete_user_friend(friend_id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::delete_user_friend(&friend_id, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/notifications")]
pub fn user_notifications(token: Result<TokenValidation, GenericError>) -> Result<Json<Notification>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match get_user_notifications(&token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(GenericError { error: true, message: err.to_string() }))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/friends")]
pub fn user_friends(token: Result<TokenValidation, GenericError>) -> Result<Json<Friends>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match get_user_friends(&token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(GenericError { error: true, message: err.to_string() }))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/users/<name>")]
pub fn search_users(name: String, token: Result<TokenValidation, GenericError>) -> Result<Json<Vec<UserSearch>>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::search_users(&name, &token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[get("/account")]
pub fn account(token: Result<TokenValidation, GenericError>) -> Result<Json<UserAccount>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::user::account(&token_data.token_iduser) {
                Ok(result) => Ok(Json(result)),
                Err(err) => Err(Json(err))
            }
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}