use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/chat/group", data="<group_info>", format="json")]
pub fn create_group(group_info: Json<GroupInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<Group>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::create_group(&group_info, &token_data.token_iduser) {
                Ok(group) => Ok(Json(group)),
                Err(err) => Err(Json(err))
            }
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}

#[put("/chat/group/<id>", data="<group_info>", format="json")]
pub fn update_group(id: String, group_info: Json<GroupInputCreate>, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::update_group(&id, &group_info, &token_data.token_iduser) {
                Ok(group) => Ok(Json(group)),
                Err(err) => Err(Json(err))
            }
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}

#[delete("/chat/group/<id>")]
pub fn delete_group(id: String, token: Result<TokenValidation, GenericError>) -> Result<Json<GenericError>, Json<GenericError>> {
    match token {
        Ok(token_data) => {
            match services::chat::delete_group(&id, &token_data.token_iduser) {
                Ok(group) => Ok(Json(group)),
                Err(err) => Err(Json(err))
            }
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}