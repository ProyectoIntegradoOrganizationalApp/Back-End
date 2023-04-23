use crate::models::models::*;
use crate::utilities::jwt::*;
use rocket::request::*;
use rocket::http::Status;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = GenericError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(token) => {
                // validate token and extract username
                let auth = validate_token(token);

                print!("{:#?}", auth);

                if auth.0 == false {
                    return Outcome::Failure((Status::BadRequest, 
                        GenericError {
                            error: true,
                            message: auth.1
                        }
                    ));
                }
                // return authenticated user
                Outcome::Success(Token {token: token.to_string()} )
            }
            None => Outcome::Failure((Status::BadRequest, 
                        GenericError {
                            error: true,
                            message: String::from("You need to provide a token")
                        }
                    )),
        }
    }
}