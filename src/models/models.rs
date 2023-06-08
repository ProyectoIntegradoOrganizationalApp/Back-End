use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable, Associations, Identifiable, AsChangeset};
use rocket::serde::{Serialize, Deserialize};
use rocket_validation::Validate;
use validator::Validate as Validate2;
use crate::schema::*;
use bigdecimal::BigDecimal;
use diesel::QueryableByName;

// TABLE'S STRUCTS ········ START
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq, AsChangeset, QueryableByName)]
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
    pub photo: String
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = role)]
pub struct Role {
    pub id: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(table_name = achievement)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub category: String,
    pub states: Vec<Option<i32>>
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Achievement, foreign_key = idachievement))]
#[diesel(primary_key(idachievement, iduser))]
#[diesel(table_name = achievement_user)]
pub struct UserAchievement {
    pub idachievement: String,
    pub iduser: String,
    pub progress: i32,
    pub percentage: BigDecimal,
    pub current_state: i32,
    pub completed: bool
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset, QueryableByName)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idproject))]
#[diesel(table_name = projects)]
pub struct Project {
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub state: i16,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(id))]
#[diesel(table_name = app)]
pub struct App {
    pub id: String,
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub photo: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(belongs_to(App, foreign_key = idapp))]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = task_app)]
pub struct TaskApp {
    pub idapp: String,
    pub idproject: String,
    pub iduser: String,
    pub app_type: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = docs_app)]
pub struct DocsApp {
    pub idapp: String,
    pub idproject: String,
    pub iduser: String,
    pub app_type: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(App, foreign_key = idapp))]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(id))]
#[diesel(table_name = board)]
pub struct Board {
    pub id: String,
    pub idapp: String,
    pub iduser: String,
    pub title: String,
    pub photo: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(Board, foreign_key = idboard))]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(id))]
#[diesel(table_name = columna)]
pub struct Columna {
    pub id: String,
    pub idboard: String,
    pub iduser: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize)]
pub struct ColumnTasks {
    pub id: String,
    pub title: String,
    pub tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(App, foreign_key = idcolumn))]
#[diesel(primary_key(id))]
#[diesel(table_name = task)]
pub struct Task {
    pub id: String,
    pub idcolumn: String,
    pub iduser: String,
    pub title: String,
    pub description: Option<String>,
    pub state: i16,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idproject, iduser))]
#[diesel(table_name = project_user)]
pub struct UserProject {
    pub idproject: String,
    pub iduser: String,
    pub idrole: String
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idproject, idguest, iduser))]
#[diesel(table_name = user_invitation)]
pub struct UserInvitation {
    pub idproject: String,
    pub idguest: String, 
    pub iduser: String,
    pub title: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idfriend, iduser))]
#[diesel(table_name = user_friend)]
pub struct UserFriend {
    pub iduser: String,
    pub idfriend: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idguest, iduser))]
#[diesel(table_name = user_friend_invitation)]
pub struct UserFriendInvitation {
    pub idguest: String, 
    pub iduser: String,
    pub title: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = idsender))]
#[diesel(primary_key(id))]
#[diesel(table_name = message)]
pub struct Message {
    pub id: String,
    pub idsender: String,
    pub idfriend: Option<String>,
    pub idgroup: Option<String>,
    pub content: String,
    pub sent_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(belongs_to(Group, foreign_key = idgroup))]
#[diesel(primary_key(iduser, idgroup))]
#[diesel(table_name = group_user)]
pub struct GroupUser {
    pub iduser: String,
    pub idgroup: String,
    pub joined_at: NaiveDateTime
}

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
    pub confirmpass: String,
    #[validate(url(message = "Must be a valid photo url"))]
    pub photo: String
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserLogin {
    #[validate(email(message = "Must be a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
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
    pub _token: String
}
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserUpdate {
    #[validate(email(message = "Must be a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
    pub password: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub name: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub lastname: String,
    #[validate(phone(message = "Must be a valid phone"))]
    pub phone: String,
    #[validate(url(message = "Must be a valid photo url"))]
    pub photo: String
}
// USER CRUD & LOGIN ········ END

// RECOVERY PASSWORD ········· START
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserMail {
    #[validate(email(message = "Must be a valid email"))]
    pub email: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
    pub message: String
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ChangePass {
    #[validate(email(message = "Must be a valid email"))]
    pub mail: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
    #[validate(must_match(other = "confirm_pass", message = "Passwords don't match"))]
    pub pass: String,
    #[validate(length(min = 8, max = 24, message = "Password length must be between 8 and 24 characters"))]
    #[validate(must_match(other = "pass", message = "Passwords don't match"))]
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
    pub achievements: Vec<UserAchievementsInfo>
}

// ACHIEVEMENTS ENDPOINT ········· END

// PROFILE ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub user: UserInfoResponse,
    pub achievements: Vec<UserAchievementsInfo>,
    pub projects: Vec<UserProjectsDetail>,
    pub owner : bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAchievementsInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub category: String,
    pub progress: i32,
    pub completed: bool,
    pub current_state: i32,
    pub percentage: BigDecimal
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub level: i16,
    pub photo: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserNotificationProfile {
    pub id: String,
    pub title: String,
    pub content: String,
    pub state: bool,
}
// PROFILE ENDPOINT ········· END

// PROJECT ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ProjectInputCreate {
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub name: String,
    #[validate(length(min = 10, max = 150, message = "Lenght must be between 10 and 150 characters"))]
    pub description: String,
    #[validate(url(message = "Must be a valid icon url"))]
    pub icon: String,
    #[validate(range(min = 1, max = 2, message = "Number range must be between 1 and 2"))]
    pub state: i16
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDetail {
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub created_at: String,
    pub updated_at: String,
    pub members: Vec<ProjectMembers>,
    pub owner: bool
}
// PROJECT ENDPOINT ········· END

// APPS AND SPECIFICATIONS ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug, Validate2)]
pub struct AppInputCreate {
    pub name: String,
    pub description: String,
    #[validate(url(message = "Must be a valid photo url"))]
    pub photo: String,
    pub apptype: String,
    #[validate]
    pub task_app: Option<TaskAppInputCreate>,
    #[validate]
    pub docs_app: Option<DocsAppInputCreate>,
}
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TaskAppInputCreate {
    pub app_type: String
}
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct DocsAppInputCreate {
    pub app_type: String
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct BoardInputCreate {
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub title: String,
    #[validate(url)]
    pub photo: String
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ColumnInputCreate {
    pub idboard: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub title: String
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TaskInputCreate {
    pub idcolumn: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub title: String,
    #[validate(length(min = 10, max = 150, message = "Lenght must be between 10 and 150 characters"))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 1, message = "Number range must be between 0 and 1"))]
    pub state: i16
}
// APPS AND SPECIFICATIONS ENDPOINT ········· END

// USER PROJECTS ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProjects {
    pub projects: Vec<UserProjectsDetail>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProjectsDetail {
    pub idproject: String,
    pub name: String, 
    pub description: String,
    pub icon: String,
    pub updated_at: String,
    pub members: Vec<ProjectMembers>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMembers {
    pub id: String,
    pub name: String,
    pub photo: String,
    pub idrole: String,
    pub role: String
}
// USER PROJECTS ENDPOINT ········· END

// USER INVITATION TO A PROJECT MESSAGE ········· START
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct InvitationMessage {
    #[validate(length(min = 3, max = 50, message = "Title lenght must be between 3 and 50 characters"))]
    pub title: String,
    #[validate(length(min = 10, max = 150, message = "Message lenght must be between 10 and 150 characters"))]
    pub message: String
}
// USER INVITATION TO A PROJECT MESSAGE ········· END

// USER ROLE IN PROJECT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct NewRole {
    pub idrole: String
}
// USER ROLE IN PROJECT ········· END

// USER NOTIFICATIONS ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub friends: Vec<UserFriendInvitation>,
    pub projects: Vec<UserInvitation>
}
// USER NOTIFICATIONS ········· END

// USER FRIENDS ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserFriends {
    pub idfriend: String,
    pub name: String,
    pub photo: String
}
// USER FRIENDS ········· END

// USER SEARCH ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSearch {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub level: i16,
    pub photo: String
}
// USER SEARCH ········· END

// MESSAGE CRUD ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageInput {
    pub idfriend: Option<String>,
    pub idgroup: Option<String>,
    pub content: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageUpdate {
    pub content: String
}
// MESSAGE CRUD ········· END

// PROJECT SEARCH ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSearch {
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub state: i16
}
// PROJECT SEARCH ········· END

// USER ACCOUNT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    pub iduser: String,
    pub name: String,
    pub lastname: String,
    pub phone: String,
    pub email: String
}
// USER ACCOUNT ········· END