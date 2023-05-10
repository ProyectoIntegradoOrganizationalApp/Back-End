use diesel::sql_types::{SmallInt, Integer};
use diesel::{Insertable, Queryable};
use diesel::pg::data_types::PgNumeric;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::*;

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
    pub id: String,
    pub full_name: String,
    pub _token: String,
    pub email: String
}

// LOGIN --------- END

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericError {
    pub error: bool,
    pub message: String
}


// MIDDLEWARES STRUCTS --------- START

#[derive(Debug, Serialize)]
pub struct TokenValidation {
   pub success: bool,
   pub message: String,
   pub token: String,
   pub owner: bool
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


// ACHIEVEMENTS --------- START
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = achievement)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = achievement_user)]
pub struct UserAchievement {
    idachievement: String,
    iduser: String,
    progress: i16,
    completed: bool,
}

// ACHIEVEMENTS --------- END

// ACHIEVEMENTS RESPONSES --------- START
#[derive(Serialize, Deserialize, Debug)]
pub struct AllAchievementsResponse {
    pub total: usize,
    pub achievements: Vec<Achievement>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAchievementsResponse {
    pub total: usize,
    pub achievements: Vec<UserAchievement>
}

// ACHIEVEMENTS RESPONSES --------- END

