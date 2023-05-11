extern crate redis;
use rust_api_rest::establish_connection;
use diesel::prelude::*;
use crate::models::models::*;
use crate::schema::{users, achievement, achievement_user, notification};

pub fn profile(id_string: &String) -> Result<Vec<(User, UserAchievement, Achievement, Notification)>, String> {
    let connection = &mut establish_connection();
    let data = users::table
    .inner_join(achievement_user::table.on(achievement_user::iduser.eq(users::id)))
    .inner_join(achievement::table.on(achievement::id.eq(achievement_user::idachievement)))
    .inner_join(notification::table.on(notification::iduser.eq(users::id)))
    // .group_by(users::id)
    .select((User::as_select(), UserAchievement::as_select(), Achievement::as_select(), Notification::as_select()))
    .filter(users::id.eq(&id_string))
    .load::<(User, UserAchievement, Achievement, Notification)>(connection);

    match data {
        Ok(result) => {
            Ok(result)
        },
        Err(err) => Err(err.to_string())
    }
}
