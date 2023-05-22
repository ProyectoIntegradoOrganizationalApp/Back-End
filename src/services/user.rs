extern crate redis;

use rust_api_rest::establish_connection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use crate::models::models::*;
use crate::schema::{users, achievement, projects, project_user, user_invitation};

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
                level: user.level,
                photo: user.photo.clone()
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
                            completed: i.1.completed,
                            current_state: i.1.current_state
                        };
                        achievements_info.push(user_achievements_info);
                    }
                    let projects_found = UserProject::belonging_to(&user)
                    .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                    .select(Project::as_select())
                    .load::<Project>(connection);
                    
                    match projects_found {
                        Ok(projects) => {
                            let mut projects_info:Vec<UserProjectProfile> = Vec::new();
                            for project in &projects {
                                let members_found = UserProject::belonging_to(&project)
                                    .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                                    .select(User::as_select())
                                    .load::<User>(connection);
                                let mut project_members:Vec<ProjectMembers> = Vec::new();
                                match members_found {
                                    Ok(members) => {
                                        for member in &members {
                                            let project_members_info = ProjectMembers {
                                                id: member.id.clone(),
                                                name: member.name.clone(),
                                                photo: member.photo.clone()
                                            };
                                            project_members.push(project_members_info);
                                        }
                                    },
                                    Err(_) => ()
                                };
                                let user_projects_info = UserProjectProfile {
                                    id: project.idproject.clone(),
                                    name: project.name.clone(),
                                    description: project.description.clone(),
                                    updated_at: project.updated_at.clone(),
                                    members: project_members
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

pub fn invite_user_to_project(guest_id: &String, project_id: &String, user_id: &String, invitation: &InvitationMessage) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let project_found = projects::table
        .select(Project::as_select())
        .filter(projects::idproject.eq(&project_id))
        .get_result::<Project>(connection);
    match project_found {
        Ok(project) => {
            let user_in_project_found = UserProject::belonging_to(&project)
                .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                .select(User::as_select())
                .filter(project_user::iduser.eq(&user_id))
                .filter(project_user::idrole.eq("1"))
                .get_result::<User>(connection);
            match user_in_project_found {
                Ok(_user) => {
                    let guest_in_project_found = UserProject::belonging_to(&project)
                        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                        .select(User::as_select())
                        .filter(project_user::iduser.eq(&guest_id))
                        .get_result::<User>(connection);
                    match guest_in_project_found {
                        Ok(_guest) => Ok(GenericError { error: true, message: "The invited user is already in the project".to_string()}),
                        Err(_err) => {
                            let new_invitation = UserInvitation {
                                idproject: project_id.clone(),
                                idguest: guest_id.clone(),
                                iduser: user_id.clone(),
                                title: invitation.title.clone(),
                                message: invitation.message.clone()
                            };
                            let created_invitation = diesel::insert_into(user_invitation::table)
                                .values(&new_invitation)
                                .get_result::<UserInvitation>(connection);
                            match created_invitation {
                                Ok(_result) => Ok(GenericError { error: false, message: "Invitation sent successfully".to_string() }),
                                Err(err) => Err(GenericError { error: true, message: err.to_string() })
                            }
                        }
                    }
                },
                Err(_err) => Err(GenericError { error: true, message: "You are not member of the project or you don't have enough privileges to invite anyone".to_string() })
            }
        },
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}

pub fn change_role_user_project(guest_id: &String, project_id: &String, user_id: &String, role: &NewRole) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let project_found = projects::table
        .select(Project::as_select())
        .filter(projects::idproject.eq(&project_id))
        .get_result::<Project>(connection);
    if guest_id != user_id {
        match project_found {
            Ok(project) => {
                let user_in_project_found = UserProject::belonging_to(&project)
                    .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                    .select(User::as_select())
                    .filter(project_user::iduser.eq(&user_id))
                    .filter(project_user::idrole.eq("1"))
                    .get_result::<User>(connection);
                match user_in_project_found {
                    Ok(_user) => {
                        let guest_in_project_found = UserProject::belonging_to(&project)
                            .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                            .select(UserProject::as_select())
                            .filter(project_user::iduser.eq(&guest_id))
                            .get_result::<UserProject>(connection);
                        match guest_in_project_found {
                            Ok(mut guest) => {
                                let idrole_number = role.idrole.clone().parse::<i32>();
                                match idrole_number {
                                    Ok(number) => {
                                        if number > 0 && number < 3 {
                                            guest.idrole = role.idrole.clone();
                                            let updated_user_role = guest.save_changes::<UserProject>(connection);
                                            match updated_user_role {
                                                Ok(_user) => Ok(GenericError {error: false, message: "User's role successfully changed".to_string()}), 
                                                Err(err) => Err(GenericError {error: true, message: err.to_string()})
                                            }
                                        } else {
                                            Err(GenericError { error: true, message: "Invalid role".to_string() })
                                        }
                                    },
                                    Err(err) => Err(GenericError {error: true, message: err.to_string()})
                                }
                            },
                            Err(_err) => Err(GenericError { error: true, message: "The invited user is not in the project".to_string()})
                        }
                    },
                    Err(_err) => Err(GenericError { error: true, message: "You are not member of the project or you don't have enough privileges to do it".to_string() })
                }
            },
            Err(err) => Err(GenericError { error: true, message: err.to_string() })
        }
    } else {
        Err(GenericError { error: true, message: "You cannot change your own role".to_string() })
    }
}