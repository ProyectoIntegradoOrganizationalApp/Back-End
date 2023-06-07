use crate::models::models::*;
use crate::utilities::jwt::*;
use rocket::{request::*, response::status};
use rocket::http::Status;

use tungstenite::http::{Request as WSRequest, Response, StatusCode};

use std::io::Cursor;
use rocket::response::Response as RocketResponse;
use rocket::http::{Status as RocketStatus, ContentType};

use tungstenite::http::Response as WsResponse;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenValidation {
    type Error = GenericError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let table = request.routed_segment(0);
        let id = request.routed_segment(1);
        match request.headers().get_one("Authorization") {
            Some(token) => {
                // Handle all the token validation
                let auth = validate_token(token, id, table);

                if auth.0 == false {
                    return Outcome::Failure((Status::BadRequest, 
                        GenericError {
                            error: true,
                            message: auth.1
                        }
                    ));
                }
                // return validated token
                Outcome::Success(TokenValidation {success: true, message: auth.1, token: auth.2, owner: auth.3, token_iduser: auth.4} )
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

pub fn ws_request_validation(req: &WSRequest<()>, res: Response<()>) -> Result<Response<()>, Response<Option<String>>> {
    let headers = req.headers();

    // Obtener el token
    if let Some(token_header) = headers.get("Sec-WebSocket-Protocol") {
        let token = token_header.to_str().unwrap();
        println!("{}", token);

        validate_token(token, None, None);
        Ok(res)
    } else{
        let response = WsResponse::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Some("TOKEN NOT FOUND WE".to_owned())).unwrap();
        Err(response)
        // Err(tungstenite::handshake::server::ErrorResponse::new(
        //     Some("Token not found".to_owned()),
        // ))
    }
}