use diesel::{Insertable, Queryable, Selectable, Associations, Identifiable, AsChangeset};
use rocket::serde::{Serialize, Deserialize};
use rocket_validation::Validate;
use crate::schema::*;

// TABLE'S STRUCTS ········ START
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub lastname: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
    pub level: i16,
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(table_name = achievement)]
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

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idproject))]
#[diesel(table_name = projects)]
pub struct Project {
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(id, idproject))]
#[diesel(table_name = app)]
pub struct App {
    pub id: String,
    pub idproject: String
}

pub enum AppTypes {
    Kanban(kanban::table),
    Docs(docs::table),
    Timeline(timeline::table),
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(belongs_to(App, foreign_key = idapp))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = kanban)]
pub struct Kanban {
    pub idapp: String,
    pub idproject: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = docs)]
pub struct Docs {
    pub idapp: String,
    pub idproject: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = timeline)]
pub struct Timeline {
    pub idapp: String,
    pub idproject: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(App, foreign_key = idapp))]
#[diesel(primary_key(id))]
#[diesel(table_name = board)]
pub struct Board {
    pub id: String,
    pub idapp: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(App, foreign_key = idboard))]
#[diesel(primary_key(id))]
#[diesel(table_name = columna)]
pub struct Columna {
    pub id: String,
    pub idboard: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(App, foreign_key = idcolumn))]
#[diesel(primary_key(id))]
#[diesel(table_name = task)]
pub struct Task {
    pub id: String,
    pub idcolumn: String,
    pub title: String,
    pub description: Option<String>,
    pub github: Option<String>
}

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
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(iduser, idproject, date))]
#[diesel(table_name = project_user_activity)]
pub struct ProjectUserActivity {
    pub iduser: String,
    pub idproject: String,
    pub date: String,
    pub commits: i16
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idproject, iduser))]
#[diesel(table_name = project_user)]
pub struct UserProject {
    pub idproject: String,
    pub iduser: String,
    pub idrole: String
}

// #[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
// #[diesel(belongs_to(User, foreign_key = iduser))]
// // #[diesel(belongs_to(User, foreign_key = idfriend))]
// #[diesel(primary_key(idfriend, iduser))]
// #[diesel(table_name = user_friend)]
// pub struct UserFriend {
//     pub iduser: String,
//     pub idfriend: String
// }

// TABLE'S STRUCTS ········ END

// USER CRUD & LOGIN ········ START

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserInput {
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub first_name: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub last_name: String,
    #[validate(phone(message = "Must be a valid phone"))]
    pub phone: String,
    #[validate(email(message = "Must be a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
    #[validate(must_match(other = "confirmpass", message = "Passwords don't match"))]
    pub password: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
    #[validate(must_match(other = "password", message = "Passwords don't match"))]
    pub confirmpass: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub lastname: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
    pub level: i16,
    pub _token: String,
    pub notifications: Vec<UserNotificationProfile>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub lastname: String,
    pub phone: String
}
// USER CRUD & LOGIN ········ END

// RECOVERY PASSWORD ········· START
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
// RECOVERY PASSWORD ········· END

// GENERIC RESPONSES ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct GenericError {
    pub error: bool,
    pub message: String
}
// GENERIC RESPONSES ········· END

// MIDDLEWARES STRUCTS ········· START

#[derive(Debug, Serialize)]
pub struct TokenValidation {
   pub success: bool,
   pub message: String,
   pub token: String,
   pub owner: bool,
   pub token_iduser: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub token: String,
    pub message: String
}

// MIDDLEWARES STRUCTS ········· END


// ACHIEVEMENTS ENDPOINT ········· START
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

// ACHIEVEMENTS ENDPOINT ········· END

// PROFILE ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub user: UserInfoResponse,
    pub achievements: Vec<UserAchievementsProfile>,
    pub projects: Vec<UserProjectProfile>,
    pub activity: Vec<UserActivityProfile>,
    pub owner : bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAchievementsProfile {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub progress: i16,
    pub completed: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub level: i16
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProjectProfile {
    pub id: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserNotificationProfile {
    pub id: String,
    pub title: String,
    pub content: String,
    pub state: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserActivityProfile {
    pub idproject: String,
    pub date: String,
    pub commits: i16
}
// PROFILE ENDPOINT ········· END

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInputCreate {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInputCreate {
    pub apptype: String,
    pub kanban: Option<KanbanInputCreate>,
    pub docs: Option<DocsInputCreate>,
    pub timeline: Option<TimelineInputCreate>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KanbanInputCreate {
    // Atributos modificables de kanban
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DocsInputCreate {
    // Atributos modificables de docs
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TimelineInputCreate {
    // Atributos modificables de timeline
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardInputCreate {
    pub idapp: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnInputCreate {
    pub idboard: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskInputCreate {
    pub idcolumn: String,
    pub title: String,
    pub description: Option<String>,
    pub github: Option<String>
}

