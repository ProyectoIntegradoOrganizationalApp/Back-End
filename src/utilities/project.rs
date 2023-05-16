use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;

pub fn is_admin(idproject: &str, iduser: &str) -> Result<UserProject, GenericError> {
    let connection = &mut establish_connection();
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
                Ok(user_project) => {
                    Ok(user_project)
                },
                Err(_) => Err(GenericError { error: true, message: "The user is not a member of the project or is not an Administrator".to_owned() })
            }
        },
        Err(_) => {
            Err(GenericError { error: true, message: "The user provided was not found".to_owned() })
        }
    }
}