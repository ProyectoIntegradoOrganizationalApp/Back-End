use diesel::{Insertable, Queryable, Selectable, Associations, Identifiable, AsChangeset};
use rocket::serde::{Serialize, Deserialize};
use rocket_validation::Validate;
use validator::Validate as Validate2;
use crate::schema::*;
use bigdecimal::BigDecimal;

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

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = iduser))]
#[diesel(primary_key(idproject))]
#[diesel(table_name = projects)]
pub struct Project {
    pub idproject: String,
    pub iduser: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(id, idproject))]
#[diesel(table_name = app)]
pub struct App {
    pub id: String,
    pub idproject: String,
    pub name: String,
    pub description: String,
    pub photo: String
}

pub enum AppTypes {
    TaskApp(task_app::table),
    DocsApp(docs_app::table)
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, AsExpression/*, AsChangeset*/)]
// #[diesel(sql_type = sql_types::ValidTaskApp)]
// pub enum ValidTaskApp {
//     Kanban,
//     Timeline
// }

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(belongs_to(App, foreign_key = idapp))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = task_app)]
pub struct TaskApp {
    pub idapp: String,
    pub idproject: String,
    pub app_type: String
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, AsExpression/*, AsChangeset*/)]
// #[sql_type="sql_types::ValidDocsApp"]
// pub enum ValidDocsApp {
//     Docs
// }

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Selectable, Associations, Identifiable, PartialEq/*, AsChangeset*/)]
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(idapp, idproject))]
#[diesel(table_name = docs_app)]
pub struct DocsApp {
    pub idapp: String,
    pub idproject: String,
    pub app_type: String
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
#[diesel(belongs_to(Project, foreign_key = idproject))]
#[diesel(primary_key(iduser, idproject, date))]
#[diesel(table_name = project_user_activity)]
pub struct ProjectUserActivity {
    pub iduser: String,
    pub idproject: String,
    pub date: String,
    pub commits: i16
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
    pub phone: String
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
    pub achievements: Vec<UserAchievement>
}

// ACHIEVEMENTS ENDPOINT ········· END

// PROFILE ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub user: UserInfoResponse,
    pub achievements: Vec<UserAchievementsProfile>,
    pub projects: Vec<UserProjectsDetail>,
    pub activity: Vec<UserActivityProfile>,
    pub owner : bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAchievementsProfile {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct UserActivityProfile {
    pub idproject: String,
    pub date: String,
    pub commits: i16
}
// PROFILE ENDPOINT ········· END

// PROJECT ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ProjectInputCreate {
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub name: String,
    #[validate(length(min = 10, max = 150, message = "Lenght must be between 10 and 150 characters"))]
    pub description: String,
    #[validate(url)]
    pub icon: String
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
    pub idapp: String,
    #[validate(length(min = 3, max = 50, message = "Lenght must be between 3 and 50 characters"))]
    pub title: String
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
    #[validate(url)]
    pub github: Option<String>
}
// APPS AND SPECIFICATIONS ENDPOINT ········· END

// USER PROJECTS ENDPOINT ········· START
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProjects {
    pub projects: Vec<UserProjectsDetail>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProjectsDetail {
    pub id: String,
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