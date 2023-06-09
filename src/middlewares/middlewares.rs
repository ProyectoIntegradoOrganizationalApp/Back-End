use crate::models::models::*;
use crate::utilities::jwt::*;
use rocket::{request::*};
use rocket::http::Status;

use tungstenite::http::{Request as WSRequest, Response, StatusCode};
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

        let auth = validate_token(token, None, None);
        if auth.0 == false {
            let response = WsResponse::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Some("Token is not valid".to_owned())).unwrap();
            return Err(response)
        }
        Ok(res)
    } else{
        println!("TOKEN NOT FOUND");
        let response = WsResponse::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Some("Token not found".to_owned())).unwrap();
        Err(response)
    }
}