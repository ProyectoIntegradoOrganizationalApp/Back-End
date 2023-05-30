use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;

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