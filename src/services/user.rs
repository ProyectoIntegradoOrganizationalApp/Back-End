extern crate redis;

use std::env;
use crate::models::models::*;
use crate::utilities::jwt::*;
use crate::utilities::redis::*;

use diesel::prelude::*;
use rust_api_rest::establish_connection;
use crate::schema::{users, achievement, achievement_user};
// use rust_api_rest::schema::achievement::dsl::*;
// use rust_api_rest::schema::achievement_user::dsl::*;

pub fn profile(id_string: &String) -> Result<(User, UserAchievement), String> {
    let connection = &mut establish_connection();
    let data = users::table
    .inner_join(achievement_user::table)
    .filter(users::columns::id.eq(&id_string))
    .first::<(User, UserAchievement)>(connection);

    match data {
        Ok(result) => {
            Ok(result)
        },
        Err(err) => Err(err.to_string())
    }
}
