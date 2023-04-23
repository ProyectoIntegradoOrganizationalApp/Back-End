use std::env;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{serde::{Serialize, Deserialize}};
use jsonwebtoken::errors::ErrorKind::*;
use rust_api_rest::establish_connection;
use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn create_token(uid: &str) -> Result<String, String> {
    let expiration_time: i64 = env::var("JWT_EXPIRATION").unwrap().parse().unwrap(); 

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration_time))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: String::from("user"),
        exp: expiration as usize,
    };
 
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let header = Header::new(Algorithm::HS512);

    match encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_bytes())) {
        Ok(token) => Ok(token),
        Err(err) => Err(err.to_string())
    }
}

pub fn validate_token(mut token: &str) -> (bool, String) {
    if !token.starts_with("Bearer") {
        return (false, "Token is no valid (Bearer)".to_string());
    }

    let collection: Vec<&str> = token.split(" ").collect();
    token = collection[1];

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );

    match decoded {
        Ok(payload) => {
            print!("TEST: {}", payload.claims.sub);
            
            let now = Utc::now().timestamp();
        
            if payload.claims.exp < now as usize{
                return (false, "The token provided is expired".to_string())
            }
            
            let connection = &mut establish_connection();

            let user_found = users.filter(id.eq(String::from(&payload.claims.sub)))
            .first::<User>(connection);
        
            match user_found{
                Ok(user) => return (true, "Login was succesful".to_string()),
                Err(err) => return (false, err.to_string())
            }

        },
        Err(err) => {
            match err.kind() {
                ExpiredSignature => (false, "The token provided is expired".to_string()),
                InvalidSignature => (false, "The token provided is invalid".to_string()),
                InvalidAlgorithm => (false, "The token provided is invalid".to_string()),
                _ => (false, err.to_string())
            }
        }
    }

}