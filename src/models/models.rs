use diesel::{Insertable, Queryable};
use rocket::serde::{Serialize, Deserialize};
use crate::schema::users;

// REGISTER --------- START

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct UserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

// REGISTER --------- END

// LOGIN --------- START

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub email: String,
    pub token: String
}

// LOGIN --------- END

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericError {
    pub error: bool,
    pub message: String
}


// MIDDLEWARES STRUCTS --------- START

#[derive(Debug)]
pub struct Token {
   pub  token: String
}

// MIDDLEWARES STRUCTS --------- END

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub token: String,
    pub message: String
}

// RECOVERY PASSWORD --------- START

#[derive(Debug, Deserialize, Serialize)]
pub struct UserMail {
    pub email: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
    pub message: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePass {
    pub mail: String,
    pub pass: String,
    pub confirm_pass: String
}
// RECOVERY PASSWORD --------- END