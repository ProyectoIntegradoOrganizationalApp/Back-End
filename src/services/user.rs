extern crate redis;

use rust_api_rest::establish_connection;
use diesel::prelude::*;
use diesel::sql_query;
use crate::models::models::*;
use crate::schema::{users, user_friend_invitation, user_friend};
use crate::utilities::achievements::*;
use crate::utilities::project::*;
use crate::utilities::user::*;

pub fn profile(id_string: &String) -> Result<UserProfile, String> {
    let connection = &mut establish_connection();
    match get_user(&id_string, connection) {
        Ok(user) => {
            // Pick up only the user's information we need
            let user_info_response = UserInfoResponse {
                id: user.id.clone(),
                name: user.name.clone(),
                email: user.email.clone(),
                level: user.level,
                photo: user.photo.clone()
            };
            
            match get_user_achievements_profile(&user, connection) {
                Ok(achievements_info) => {
                    match get_own_projects(&user, connection) {
                        Ok(projects_info) => {
                            match get_task_completed(&user.id, connection) {
                                Ok(task_completed) => {
                                    let user_profile = UserProfile {
                                        user: user_info_response,
                                        achievements: achievements_info,
                                        projects: projects_info,
                                        completed_task: task_completed,
                                        owner: false
                                    };
                                    Ok(user_profile)
                                },
                                Err(err) => Err(err.message)
                            }
                        },
                        Err(_) => Err("Error getting your own projects".to_string())
                    }
                },
                Err(err) => Err(err)
            }
        },
        Err(_) => Err("User not found".to_string())
    }
}

// USER FRIENDS MANAGEMENT
pub fn send_friend_request(guest_id: &String, invitation: &InvitationMessage, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    if user_id != guest_id {
        let guest_found = users::table
            .select(User::as_select())
            .filter(users::id.eq(&guest_id))
            .get_result::<User>(connection);
        
        match guest_found {
            Ok(_guest) => {
                let friend_found = user_friend::table
                    .select(UserFriend::as_select())
                    .filter(user_friend::idfriend.eq(&guest_id))
                    .filter(user_friend::iduser.eq(&user_id))
                    .get_result::<UserFriend>(connection);
                
                match friend_found {
                    Ok(_) => Ok(GenericError { error: true, message: "You are already a friend of this user".to_string() }),
                    Err(_) => {
                        let new_user_friend_invitation = UserFriendInvitation {
                            idguest: guest_id.clone(),
                            iduser: user_id.clone(),
                            title: invitation.title.clone(),
                            message: invitation.message.clone()
                        };
        
                        let created = diesel::insert_into(user_friend_invitation::table)
                        .values(&new_user_friend_invitation)
                        .get_result::<UserFriendInvitation>(connection);
        
                        match created {
                            Ok(_) => Ok(GenericError { error: false, message: "Friend request sent successfully".to_string() }),
                            Err(_) => Err(GenericError { error: true, message: "Error while sending friend request".to_string() })
                        }
                    }
                }
            },
            Err(_) => Err(GenericError { error: true, message: "Guest not found".to_string() })
        }
    } else {
        Err(GenericError { error: true, message: "You cannot be a friend of yourself".to_string() })
    }
}

pub fn accept_friend_request(user_id: &String, guest_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let invitation_found = user_friend_invitation::table
            .select(UserFriendInvitation::as_select())
            .filter(user_friend_invitation::idguest.eq(&guest_id))
            .filter(user_friend_invitation::iduser.eq(&user_id))
            .get_result::<UserFriendInvitation>(connection);
    
    match invitation_found {
        Ok(_invitation) => {
            let new_friend = UserFriend {
                iduser: user_id.clone(),
                idfriend: guest_id.clone()
            };

            let new_friend_guest = UserFriend {
                iduser: guest_id.clone(),
                idfriend: user_id.clone()
            };

            let created = diesel::insert_into(user_friend::table)
            .values(&new_friend)
            .get_result::<UserFriend>(connection);

            match created {
                Ok(_) => {
                    let created = diesel::insert_into(user_friend::table)
                    .values(&new_friend_guest)
                    .get_result::<UserFriend>(connection);
                    match created {
                        Ok(_) => {
                            let deleted = diesel::delete(user_friend_invitation::table.filter(user_friend_invitation::iduser.eq(&user_id)))
                            .filter(user_friend_invitation::idguest.eq(&guest_id))
                            .execute(connection);
                            match deleted {
                                Ok(_) => {
                                    let friend_invitation_found = user_friend_invitation::table
                                        .filter(user_friend_invitation::iduser.eq(&guest_id))
                                        .filter(user_friend_invitation::idguest.eq(&user_id))
                                        .select(UserFriendInvitation::as_select())
                                        .get_result::<UserFriendInvitation>(connection);
                                    match friend_invitation_found {
                                        Ok(friend_invitation) => {
                                            let deleted = diesel::delete(user_friend_invitation::table.filter(user_friend_invitation::iduser.eq(&friend_invitation.iduser)))
                                                .filter(user_friend_invitation::idguest.eq(&friend_invitation.idguest))
                                                .execute(connection);
                                            match deleted {
                                                Ok(_) => {
                                                    // Check achievement - id: 1
                                                    let achievement_updated = check_update_user_achievement(user_id, "1");
                                                    match achievement_updated {
                                                        Ok(_) => {},
                                                        Err(err) => return Err(err)
                                                    }
                                                    // Check achievement - id: 1
                                                    let achievement_updated = check_update_user_achievement(guest_id, "1");
                                                    match achievement_updated {
                                                        Ok(_) => {},
                                                        Err(err) => return Err(err)
                                                    }
                                                    Ok(GenericError { error: false, message: "You have made a new friend!".to_string() })
                                                },
                                                Err(_) => Err(GenericError { error: true, message: "Error accepting friend request".to_string() })
                                            }
                                        },
                                        Err(_err) => {
                                            // Check achievement - id: 1
                                            let achievement_updated = check_update_user_achievement(user_id, "1");
                                            match achievement_updated {
                                                Ok(_) => {},
                                                Err(err) => return Err(err)
                                            }
                                            // Check achievement - id: 1
                                            let achievement_updated = check_update_user_achievement(guest_id, "1");
                                            match achievement_updated {
                                                Ok(_) => {},
                                                Err(err) => return Err(err)
                                            }
                                            Ok(GenericError { error: false, message: "You have made a new friend!".to_string() })
                                        }
                                    }
                                },
                                Err(_) => Err(GenericError { error: true, message: "Error while deleting friend request".to_string() })
                            }
                        },
                        Err(_) => Err(GenericError { error: true, message: "Error while accepting friend request".to_string() })
                    }
                },
                Err(_) => Err(GenericError { error: true, message: "Error while making a new friend".to_string() })
            }
        }, 
        Err(_) => Err(GenericError { error: true, message: "Invitation not found".to_string() })
    }
}

pub fn deny_friend_request(user_id: &String, guest_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let invitation_found = user_friend_invitation::table
            .select(UserFriendInvitation::as_select())
            .filter(user_friend_invitation::idguest.eq(&guest_id))
            .filter(user_friend_invitation::iduser.eq(&user_id))
            .get_result::<UserFriendInvitation>(connection);
    match invitation_found {
        Ok(_invitation) => {
            let deleted = diesel::delete(user_friend_invitation::table.filter(user_friend_invitation::iduser.eq(&user_id)))
            .filter(user_friend_invitation::idguest.eq(&guest_id))
            .execute(connection);
            match deleted {
                Ok(_) => Ok(GenericError { error: false, message: "Friend request denied successfully".to_string() }),
                Err(_) => Err(GenericError { error: true, message: "Error at deny friend request".to_string() })
            }
        }, 
        Err(_) => Err(GenericError { error: true, message: "Invitation not found".to_string() })
    }
}

pub fn delete_user_friend(friend_id: &String, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let friend_found = user_friend::table
        .select(UserFriend::as_select())
        .filter(user_friend::idfriend.eq(&friend_id))
        .filter(user_friend::iduser.eq(&user_id))
        .get_result::<UserFriend>(connection);

    match friend_found {
        Ok(_friend) => {
            let deleted = diesel::delete(user_friend::table.filter(user_friend::iduser.eq(&user_id)))
                .filter(user_friend::idfriend.eq(&friend_id))
                .execute(connection);
            match deleted {
                Ok(_) => {
                    let deleted = diesel::delete(user_friend::table.filter(user_friend::iduser.eq(&friend_id)))
                    .filter(user_friend::idfriend.eq(&user_id))
                    .execute(connection);
                    
                    match deleted {
                        Ok(_) => Ok(GenericError { error: false, message: "You are no longer friends with this user".to_string() }),
                        Err(_) => Err(GenericError { error: true, message: "Error while deleting friend".to_string() })
                    }
                },
                Err(_) => Err(GenericError { error: true, message: "Error while deleting friend".to_string() })
            }
        },
        Err(_) => Err(GenericError { error: true, message: "Friend not found".to_string() })
    }
}

pub fn get_user_achievements(user_id: String) -> Result<UserAchievementsResponse, GenericError> {
    let connection = &mut establish_connection();
    match get_user(&user_id, connection) {
        Ok(user) => {
            match get_user_achievements_util(&user, connection) {
                Ok(achievements) => { 
                    let response = UserAchievementsResponse {
                        total: achievements.len(),
                        achievements
                    };
                    Ok(response)
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(GenericError { error: true, message: err })
    }
}

pub fn search_users(name_str: &String, user_id: &String) -> Result<Users, GenericError> {
    let connection = &mut establish_connection();
    let name = name_str.to_lowercase();
    let users_found = sql_query(format!("
        SELECT * 
        FROM users 
        WHERE LOWER(name) LIKE '%{name}%' 
        OR LOWER(lastname) LIKE '%{name}%'
        LIMIT 10
    ")).load::<User>(connection);
    match users_found {
        Ok(users) => {
            let mut users_search:Vec<UserSearch> = Vec::new();
            for user in &users {
                if user.id != user_id.to_owned() && !is_friend(&user.id, &user_id, connection) {
                    let new_user_search = UserSearch {
                        id: user.id.clone(),
                        name: user.name.clone(),
                        lastname: user.lastname.clone(),
                        email: user.email.clone(),
                        level: user.level,
                        photo: user.photo.clone()
                    };
                    users_search.push(new_user_search);
                }
            }
            let users_response = Users {
                users: users_search
            };
            Ok(users_response)
        },
        Err(_) => Err(GenericError { error: true, message: "Users not found".to_string() })
    }
}

pub fn account(user_id: &String) -> Result<UserAccount, GenericError> {
    let connection = &mut establish_connection();
    match get_user(&user_id, connection) {
        Ok(user) => {
            let user_account = UserAccount {
                iduser: user.id.clone(),
                name: user.name.clone(),
                lastname: user.lastname.clone(),
                phone: user.phone.clone(),
                email: user.email.clone(),
                photo: user.photo.clone()
            };
            Ok(user_account)
        },
        Err(_) => Err(GenericError { error: true, message: "User not found".to_string() })
    }
}

// pub fn most_valued_in_project(project_id: &String) -> Result<GenericError, GenericError> {
//     let connection = &mut establish_connection();
//     let most_valued_found = sql_query(format!("
//         SELECT iduser, COUNT(id) AS total
//         FROM task
//         GROUP BY iduser
//         HAVING idproject = '{project_id}' AND state = 1
//         ORDER BY iduser DESC
//     ")).load::<String>(connection);
//     match most_valued_found {
//         Ok(task) => {
//             Ok(GenericError { error: true, message: "idk".to_string() })
//         },
//         Err(err) => Err(GenericError { error: true, message: err.to_string() })
//     }
// }