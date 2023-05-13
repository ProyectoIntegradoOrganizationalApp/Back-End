use crate::models::models::*;
use crate::utilities::jwt::*;
use rocket::request::*;
use rocket::http::Status;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenValidation {
    type Error = GenericError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let method = request.method();
        let id = request.routed_segment(1);
        println!("{}", method);
        match request.headers().get_one("Authorization") {
            Some(token) => {
                // Handle all the token validation
                let auth = validate_token(token, id);

                if auth.0 == false {
                    return Outcome::Failure((Status::BadRequest, 
                        GenericError {
                            error: true,
                            message: auth.1
                        }
                    ));
                }
                // return validated token
                Outcome::Success(TokenValidation {success: true, message: auth.1, token: auth.2, owner: auth.3} )
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