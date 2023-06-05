use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub fn group_exists(group_id: &String, connection: &mut PgConnection) -> bool {
    let group_found = groups::table.filter(groups::id.eq(group_id)).first::<Group>(connection);
    match group_found {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn is_group_member(user_id: &String, group_id: &String, connection: &mut PgConnection) -> bool {
    let user_found = group_user::table.filter(group_user::idgroup.eq(group_id))
        .filter(group_user::iduser.eq(user_id)).first::<GroupUser>(connection);
    match user_found {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_message(message_id: &String, user_id: &String, connection: &mut PgConnection) -> Result<Message, GenericError> {
    let found_message = message::table.filter(message::id.eq(message_id))
        .filter(message::idsender.eq(user_id))
        .first::<Message>(connection);
    match found_message {
        Ok(message) => {
            Ok(message)
        },
        Err(_) => Err(GenericError { error: true, message: "Couldn't find that message for the user".to_owned() })
    }
}