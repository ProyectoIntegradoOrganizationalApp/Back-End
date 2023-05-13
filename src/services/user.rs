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
            // Pick up only the user's information we need
            let user_info_response = UserInfoResponse {
                id: user.id.clone(),
                name: user.name.clone(),
                email: user.email.clone(),
                level: user.level
            };
            let achievements_found = UserAchievement::belonging_to(&user)
            .inner_join(achievement::table)
            .select((Achievement::as_select(), UserAchievement::as_select()))
            .load::<(Achievement, UserAchievement)>(connection);
            
            match achievements_found {
                Ok(achievements) => {
                    // Pick up only the achievements' information we need
                    let mut achievements_info:Vec<UserAchievementsProfile> = Vec::new();
                    for i in &achievements {
                        let user_achievements_info = UserAchievementsProfile {
                            id: i.0.id.clone(),
                            title: i.0.title.clone(),
                            description: i.0.description.clone(),
                            icon: i.0.icon.clone(),
                            progress: i.1.progress,
                            completed: i.1.completed
                        };
                        achievements_info.push(user_achievements_info);
                    }
                    let projects_found = UserProject::belonging_to(&user)
                    .inner_join(projects::table.on(project_user::id.eq(projects::id)))
                    .select(Project::as_select())
                    .load::<Project>(connection);
                    
                    match projects_found {
                        Ok(projects) => {
                            let mut projects_info:Vec<UserProjectProfile> = Vec::new();
                            for i in &projects {
                                let user_projects_info = UserProjectProfile {
                                    id: i.id.clone(),
                                    name: i.name.clone()
                                };
                                projects_info.push(user_projects_info);
                            }

                            let activity_found = ProjectUserActivity::belonging_to(&user)
                                    .select(ProjectUserActivity::as_select())
                                    .load::<ProjectUserActivity>(connection);
                                    
                            match activity_found {
                                Ok(activity) => {
                                    let mut activity_info:Vec<UserActivityProfile> = Vec::new();
                                    for i in &activity {
                                        let user_activity_info = UserActivityProfile {
                                            idproject: i.idproject.clone(),
                                            date: i.date.clone(),
                                            commits: i.commits
                                        };
                                        activity_info.push(user_activity_info);
                                    }
                                    let user_profile = UserProfile {
                                        user: user_info_response,
                                        achievements: achievements_info,
                                        projects: projects_info,
                                        activity: activity_info,
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
