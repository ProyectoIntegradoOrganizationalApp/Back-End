use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;
use diesel::result::Error;

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