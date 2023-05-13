use crate::models::models::*;
use crate::schema::users::dsl::*;
use crate::utilities::redis::*;
use chrono::prelude::*;
use diesel::prelude::*;
use jsonwebtoken::errors::ErrorKind::*;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rocket::serde::{Deserialize, Serialize};
use rust_api_rest::establish_connection;
use std::env;

extern crate redis;

// Token Claims (Payload) structure
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn create_token(uid: &str) -> Result<String, String> {
    // Get the expiration time from env variable
    let expiration_time: i64 = env::var("JWT_EXPIRATION").unwrap().parse().unwrap();

    // Transform to DateTime format
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration_time))
        .expect("valid timestamp")
        .timestamp();

    // Transform to DateTime format
    let claims = Claims {
        sub: uid.to_owned(),
        role: String::from("user"),
        exp: expiration as usize,
    };

    // Get private key from env variable
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let header = Header::new(Algorithm::HS512);

    // Encode the token
    match encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(err) => Err(err.to_string()),
    }
}

#[allow(unused)]
pub fn validate_token(mut token: &str, request_id: Option<&str>) -> (bool, String, String, bool) {
    token = get_token(&token);
    if (token == "") {
        return (
            false,
            "Token is no valid (Bearer)".to_string(),
            "".to_string(),
            false
        );
    }

    let decoded = decode_token(&token);

    match decoded {
        Ok(payload) => {
            let now = Utc::now().timestamp();

            // Check expiration
            if payload.claims.exp < now as usize {
                unwhitelist_token(token);
                return (
                    false,
                    "The token provided is expired".to_string(),
                    "".to_string(),
                    false
                );
            }

            let connection = &mut establish_connection();

            let user_found = users
                .filter(id.eq(String::from(&payload.claims.sub)))
                .first::<User>(connection);

            // Check if the user doing the request is the database
            match user_found {
                Ok(user) => (),
                Err(err) => return (false, err.to_string(), "".to_string(), false),
            }

            // Check if the token is whitelisted
            match get_whitelist_token(token) {
                Ok(user_id) => {
                    // Check if the user that created the token is equal to the one doing the request
                    if (user_id != String::from(&payload.claims.sub)) {
                        return (
                            false,
                            "You are not the owner of that token!".to_string(),
                            "".to_string(),
                            false
                        );
                    }
                    // If requesting id is not "None" it means that user is trying to access to a single user route
                    // Example: /profile/<id>
                    // Check if requesting id is valid
                    if (request_id != None) {
                        let request_id_str = request_id.as_deref().unwrap_or("default string");
                        // The variable "owner" will determine your permissions and the information you can see.
                        // If you are the owner you can change or delete information of all that owns you.
                        let user_found = users
                            .filter(id.eq(String::from(request_id_str)))
                            .first::<User>(connection);
                        match user_found {
                            Ok(user) => {
                                let mut owner = false;
                                let mut message = "The requesting id is not valid";
                                if (user_id == request_id_str) {
                                    owner = true;
                                    message = "The requesting id is valid";
                                }
                                return (
                                    true,
                                    message.to_string(),
                                    token.to_string(),
                                    owner
                                );
                            }
                            Err(err) => {
                                return (
                                    false,
                                    "The requesting id is not valid".to_string(),
                                    "".to_string(),
                                    false
                                )
                            }
                        }
                    }
                    // If the code gets here it means the middleware has triggered in /login endpoint
                    return (true, "Login was succesful".to_string(), token.to_string(), true);
                }
                Err(err) => {
                    return (
                        false,
                        "The token provided is not whitelisted".to_string(),
                        "".to_string(),
                        false
                    )
                }
            }
        }
        Err(err) => match err.kind() {
            ExpiredSignature => {
                unwhitelist_token(token);
                (
                    false,
                    "The token provided is expired".to_string(),
                    "".to_string(),
                    false
                )
            }
            InvalidSignature => (
                false,
                "The token provided is invalid".to_string(),
                "".to_string(),
                false
            ),
            InvalidAlgorithm => (
                false,
                "The token provided is invalid".to_string(),
                "".to_string(),
                false
            ),
            _ => (false, err.to_string(), "".to_string(), false),
        },
    }
}

#[allow(unused)]
fn get_token(mut token: &str) -> &str {
    if !token.starts_with("Bearer") {
        return ("");
    }

    // Remove the "Bearer" word from the string
    let collection: Vec<&str> = token.split(" ").collect();
    token = collection[1];

    token
}

#[allow(unused)]
fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    // Get private key
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Decode the token to get the payload
    let decoded: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );

    decoded
}
