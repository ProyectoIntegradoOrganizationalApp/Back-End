use std::env;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{serde::{Serialize, Deserialize}};
use jsonwebtoken::errors::ErrorKind::*;
use rust_api_rest::establish_connection;
use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use chrono::prelude::*;
use crate::utilities::redis::*;

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
    match encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_bytes())) {
        Ok(token) => Ok(token),
        Err(err) => Err(err.to_string())
    }
}

#[allow(unused)]
pub fn validate_token(mut token: &str) -> (bool, String) {

    // Check if token is Bearer
    if !token.starts_with("Bearer") {
        return (false, "Token is no valid (Bearer)".to_string());
    }

    // Remove the "Bearer" word from the string
    let collection: Vec<&str> = token.split(" ").collect();
    token = collection[1];

    // Get private key
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    // Decode the token to get the payload
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );
    
    match decoded {
        Ok(payload) => {
            let now = Utc::now().timestamp();
        
            // Check expiration
            if payload.claims.exp < now as usize{
                unwhitelist_token(token);
                return (false, "The token provided is expired".to_string())
            }

            let connection = &mut establish_connection();
            let user_found = users.filter(id.eq(String::from(&payload.claims.sub)))
            .first::<User>(connection);
        
            // Check if the user doing the request is the database
            match user_found{
                Ok(user) => (),
                Err(err) => return (false, err.to_string())
            }

            // Check if the token is whitelisted
            match get_whitelist_token(token) {
                Ok(user_id) => {
                    // Check if the user that created the token is equal to the one doing the request
                    if(user_id != String::from(&payload.claims.sub)) {
                        return (false, "You are not the owner of that token!".to_string())
                    }
                    return (true, "Login was succesful".to_string())
                },
                Err(err) => return (false, "The token provided is not whitelisted".to_string())
            }
            
        },
        Err(err) => {
            match err.kind() {
                ExpiredSignature => {
                    unwhitelist_token(token);
                    (false, "The token provided is expired".to_string())
                },
                InvalidSignature => (false, "The token provided is invalid".to_string()),
                InvalidAlgorithm => (false, "The token provided is invalid".to_string()),
                _ => (false, err.to_string())
            }
        }
    }

}