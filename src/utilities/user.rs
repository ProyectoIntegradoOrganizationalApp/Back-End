use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;
use diesel::result::Error;
use diesel::dsl::count;

pub fn get_user(user_id: &String, connection: &mut PgConnection) -> Result<User, String> {
    let user_found = users::table
        .select(User::as_select())
        .filter(users::id.eq(&user_id))
        .get_result::<User>(connection);

    match user_found {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string())
    }
}

pub fn is_member(idproject: &str, iduser: &str, connection: &mut PgConnection) -> bool {
    let user_found: Result<User, Error> = users::table.filter(users::id.eq(&iduser)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let user_project_found: Result<UserProject, Error> = UserProject::belonging_to(&user)
                .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                .filter(projects::idproject.eq(idproject))
                .select(UserProject::as_select())
                .get_result::<UserProject>(connection);
            match user_project_found {
                Ok(_) => true,
                Err(_) => false
            }
        },
        Err(_) => false
    }
}

pub fn is_admin(idproject: &str, iduser: &str, connection: &mut PgConnection) -> bool {
    let user_found: Result<User, Error> = users::table.filter(users::id.eq(&iduser)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let user_project_found: Result<UserProject, Error> = UserProject::belonging_to(&user)
                .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                .filter(projects::idproject.eq(idproject))
                .filter(project_user::idrole.eq("1".to_string()))
                .select(UserProject::as_select())
                .get_result::<UserProject>(connection);
            match user_project_found {
                Ok(_) => true,
                Err(_) => false
            }
        },
        Err(_) => false
    }
}

pub fn is_editor(idproject: &str, iduser: &str, connection: &mut PgConnection) -> bool {
    let user_found: Result<User, Error> = users::table.filter(users::id.eq(&iduser)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let user_project_found: Result<UserProject, Error> = UserProject::belonging_to(&user)
                .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                .filter(projects::idproject.eq(idproject))
                .filter(project_user::idrole.eq("2".to_string()))
                .select(UserProject::as_select())
                .get_result::<UserProject>(connection);
            match user_project_found {
                Ok(_) => true,
                Err(_) => false
            }
        },
        Err(_) => false
    }
}
#[allow(unused)]
pub fn is_friend(user_id: &String, friend_id: &String, connection: &mut PgConnection) -> bool {
    let friend_found = user_friend::table
        .filter(user_friend::iduser.eq(&user_id).and(user_friend::idfriend.eq(&friend_id)))
        .or_filter(user_friend::iduser.eq(&friend_id).and(user_friend::idfriend.eq(&user_id)))
        .first::<UserFriend>(connection);
    match friend_found {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn level_up(user_id: &String, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
    match get_user(user_id, connection) {
        Ok(mut user) => {
            user.level += 1;
            let user_updated = user.save_changes::<User>(connection);
            match user_updated {
                Ok(_) => Ok(GenericError { error: false, message: "Level up!".to_string() }),
                Err(_) => Err(GenericError { error: true, message: "Error while leveling up".to_string() })
            }
        },
        Err(_) => Err(GenericError { error: true, message: "The user you are searching for doesn't exist".to_owned() })
    }
}

pub fn get_task_completed(user_id: &str, connection: &mut PgConnection) -> Result<i64, GenericError> {
    let number_tasks_found = task::table
        .select(count(task::id))
        .filter(task::iduser.eq(user_id))
        .filter(task::state.eq(1))
        .get_result::<i64>(connection);
    match number_tasks_found {
        Ok(number) => Ok(number),
        Err(_) => Err(GenericError { error: true, message: "Error trying to get the tasks".to_owned() })
    }
}