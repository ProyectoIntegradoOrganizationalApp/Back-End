extern crate redis;

use rust_api_rest::establish_connection;
use diesel::prelude::*;
use crate::models::models::*;
use crate::schema::{user_invitation, user_friend_invitation, user_friend, users};

pub fn get_user_notifications(user_id: &String) -> Result<Notification, String> {
    let connection = &mut establish_connection();
    let project_invitations_found = user_invitation::table
        .filter(user_invitation::idguest.eq(&user_id))
        .load::<UserInvitation>(connection);
    match project_invitations_found {
        Ok(projects) => {
            let friend_requests_found = user_friend_invitation::table
                .filter(user_friend_invitation::idguest.eq(&user_id))
                .load::<UserFriendInvitation>(connection);

            match friend_requests_found {
                Ok(friends) => {
                    let notifications = Notification {
                        friends,
                        projects
                    };
                    Ok(notifications)
                },
                Err(_) => Err("Friend requests not found".to_string())
            }
        },
        Err(_) => Err("Project invitations not found".to_string())
    }
}

pub fn get_user_friends(user_id: &String) -> Result<Friends, String> {
    let connection = &mut establish_connection();
    let friends_found = user_friend::table
        .inner_join(users::table.on(user_friend::iduser.eq(users::id)))
        .filter(user_friend::iduser.eq(&user_id))
        .select(User::as_select())
        .load::<User>(connection);
    match friends_found {
        Ok(friends) => {
            let mut friends_info:Vec<UserFriends> = Vec::new();
            for friend in &friends {
                if friend.id != user_id.clone() {
                    let friend_info = UserFriends {
                        idfriend: friend.id.clone(),
                        name: friend.name.clone(),
                        photo: friend.photo.clone()
                    };
                    friends_info.push(friend_info);
                }
            }
            let friends_found = user_friend::table
                .inner_join(users::table.on(user_friend::iduser.eq(users::id)))
                .filter(user_friend::idfriend.eq(&user_id))
                .select(User::as_select())
                .load::<User>(connection);
            match friends_found {
                Ok(friends) => {
                    for friend in &friends {
                        if friend.id != user_id.clone() {
                            let friend_info = UserFriends {
                                idfriend: friend.id.clone(),
                                name: friend.name.clone(),
                                photo: friend.photo.clone()
                            };
                            friends_info.push(friend_info);
                        }
                    }
                    let friends_response = Friends {
                        friends: friends_info
                    };
                    Ok(friends_response)
                },
                Err(_) => Err("Friends not found".to_string())
            }
        },
        Err(_) => Err("Friends not found".to_string())
    }
}