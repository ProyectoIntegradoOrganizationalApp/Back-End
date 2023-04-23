use rocket::serde::json::{Json};
use crate::models::models::*;
use crate::services;

#[post("/register", data="<user_info>", format="json")]
pub fn register(user_info: Json<UserInput>) -> Json<User> {
    Json(services::auth::register(&user_info))
}

#[post("/login", data="<user_info>", format="json")]
pub fn login(user_info: Json<UserLogin>) -> Result<Json<UserLoginResponse>, Json<GenericError>> {

    let result = match services::auth::login(&user_info) {
        Ok(token) => {
            let response = UserLoginResponse {
                email: String::from(&user_info.email),
                token: token
            };
            Ok(Json(response))
        }
        Err(err) => {
            let response = GenericError {
                error: true,
                message: String::from(err)
            };
            Err(Json(response))
        }
    };

    result
}

#[get("/test_token", format="json")]
pub fn test_token(token: Result<TokenValidantion, GenericError>) -> Result<Json<TokenValidantion>, Json<GenericError>> {

    match token {
        Ok(validation) => {
            return Ok(Json(validation))
        },
        Err(json_error) => {
            return Err(Json(json_error))
        }
    }
}

#[post("/send_mail", data="<user_mail>", format="json")]
pub fn send_mail(user_mail: Json<UserMail>) -> Json<ResponseMessage> {
    Json(services::auth::send_mail(&user_mail))
}

#[post("/change_password", data="<user_info>", format="json")]
pub fn change_password(user_info: Json<ChangePass>) -> Result<Json<ResponseMessage>, Json<GenericError>>{
    let result = match services::auth::change_password(&user_info) {
        Ok(msg) => {
            let response = ResponseMessage {
                message: msg
            };
            Ok(Json(response))
        }
        Err(err) => {
            let response = GenericError {
                error: true,
                message: String::from(err)
            };
            Err(Json(response))
        }
    };

    result
}

// #[post("/logout", data="<token>", format="json")]
// pub fn logout(token: Token) -> Result<Json<ResponseMessage>, Json<GenericError>> {

// }