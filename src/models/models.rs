use diesel::{Insertable, Queryable, Selectable, Associations, Identifiable};
use rocket::serde::{Serialize, Deserialize};
use crate::schema::*;

// REGISTER --------- START

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
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
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = achievement)]
#[diesel(primary_key(id))]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Achievement, foreign_key = idachievement))]
#[diesel(primary_key(idachievement, iduser))]
#[diesel(table_name = achievement_user)]
pub struct UserAchievement {
    pub idachievement: String,
    pub iduser: String,
    pub progress: i16,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(id))]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: String,
    pub iduser: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Project, foreign_key = id))]
#[diesel(primary_key(id, iduser))]
#[diesel(table_name = project_user)]
pub struct UserProject {
    pub id: String,
    pub iduser: String,
    pub idrole: String
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

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(id))]
#[diesel(table_name = notification)]
pub struct Notification {
    pub id: String,
    pub iduser: String,
    pub title: String,
    pub content: String,
    pub state: bool,
}
// PROFILE ENDPOINT STRUCT
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub user: User,
    pub achievements: Vec<(Achievement, UserAchievement)>,
    pub projects: Vec<Project>,
    pub notifications: Vec<Notification>,
    pub owner : bool
}