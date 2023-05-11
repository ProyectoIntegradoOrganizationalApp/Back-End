extern crate redis;
use rust_api_rest::establish_connection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use crate::models::models::*;
use crate::schema::{users, achievement, achievement_user, notification};

pub fn profile(id_string: &String) -> Result<UserAchievementsJson, String>{
    let connection = &mut establish_connection();
    let user_found = users::table
    .select(User::as_select())
    .filter(users::id.eq(&id_string))
    .get_result::<User>(connection);

    match user_found {
        Ok(user) => {
            let achievements_found = UserAchievement::belonging_to(&user)
            .inner_join(achievement::table)
            .select(Achievement::as_select())
            .load::<Achievement>(connection);
            
            match achievements_found {
                Ok(achievements) => {
                    let user_achievements = UserAchievementsJson {
                        user,
                        achievements
                    };
                    Ok(user_achievements)
                },
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}
