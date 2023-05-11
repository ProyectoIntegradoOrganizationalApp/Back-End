extern crate redis;
use rust_api_rest::establish_connection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use crate::models::models::*;
use crate::schema::{users, achievement, projects, project_user};

pub fn profile(id_string: &String) -> Result<UserProfile, String>{
    let connection = &mut establish_connection();
    let user_found = users::table
    .select(User::as_select())
    .filter(users::id.eq(&id_string))
    .get_result::<User>(connection);

    match user_found {
        Ok(user) => {
            let achievements_found = UserAchievement::belonging_to(&user)
            .inner_join(achievement::table)
            .select((Achievement::as_select(), UserAchievement::as_select()))
            .load::<(Achievement, UserAchievement)>(connection);
            
            match achievements_found {
                Ok(achievements) => {
                    let projects_found = UserProject::belonging_to(&user)
                    .inner_join(projects::table.on(project_user::id.eq(projects::id)))
                    .select(Project::as_select())
                    .load::<Project>(connection);
                    
                    match projects_found {
                        Ok(projects) => {
                            let notifications_found = Notification::belonging_to(&user)
                            .select(Notification::as_select())
                            .load::<Notification>(connection);

                            match notifications_found {
                                Ok(notifications) => {
                                    let user_profile = UserProfile {
                                        user,
                                        achievements,
                                        projects,
                                        notifications,
                                        owner: false
                                    };
                                    Ok(user_profile)
                                },
                                Err(err) => Err(err.to_string())
                            }
                        },
                        Err(err) => Err(err.to_string())
                    }
                },
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}
