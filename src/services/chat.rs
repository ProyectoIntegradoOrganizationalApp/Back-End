use crate::models::models::*;
use crate::schema::*;
use chrono::Utc;
use diesel::prelude::*;
use rust_api_rest::establish_connection;
use crate::utilities::user as user_utils;
use crate::utilities::chat as chat_utils;

pub fn create_message(user_id: &String, message_data: &MessageInput) -> Result<Message, GenericError> {
    let connection = &mut establish_connection();
    let new_message = Message {
        id:  uuid::Uuid::new_v4().to_string(),
        idsender: user_id.clone(),
        idfriend: message_data.idfriend.clone(),
        idgroup: message_data.idgroup.clone(),
        content: message_data.content.clone(),
        sent_at: Utc::now().naive_utc()
    };

    // Casos de errores previos a la creación del mensajes
    if new_message.idfriend.is_some() && new_message.idgroup.is_some() {
        return Err(GenericError { error: true, message: "The message cant be assigned to a private and group environment at the same time".to_owned() })
    } else if new_message.idfriend.is_none() && new_message.idgroup.is_none() {
        return Err(GenericError { error: true, message: "The message must be assigned to at least one evironment".to_owned() })
    } else if new_message.idfriend.is_some() && !user_utils::is_friend(user_id, new_message.idfriend.as_ref().unwrap(), connection) {
        return Err(GenericError { error: true, message: "You cant send a message to someone that is not your friend!".to_owned() })
    } else if new_message.idgroup.is_some() && !chat_utils::group_exists(new_message.idgroup.as_ref().unwrap(), connection) {
        return Err(GenericError { error: true, message: "The message must be a sent to a valid group!".to_owned() })
    }

    let created_message = diesel::insert_into(message::table)
        .values(&new_message)
        .get_result::<Message>(connection);

    match created_message {
        Ok(message) => Ok(message),
        Err(_) => Err(GenericError { error: true, message: "An error ocurred creating your message".to_owned() })
    }
}

pub fn update_message(message_id: &String, user_id: &String, message_data: &MessageUpdate) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match chat_utils::get_message(message_id, user_id, connection) {
        Ok(mut message) => {
            message.content = message_data.content.clone();
            let updated_message = message.save_changes::<Message>(connection);
            match updated_message {
                Ok(_) => Ok(GenericError { error: false, message: "Message successfully updated".to_owned() }),
                Err(_) => Err(GenericError { error: true, message: "An error ocurred updating the message".to_owned() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn delete_message(message_id: &String, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match chat_utils::get_message(message_id, user_id, connection) {
        Ok(message) => {
            let deleted_message = diesel::delete(message::table)
                .filter(message::id.eq(message.id))
                .execute(connection);
            match deleted_message {
                Ok(_) => Ok(GenericError { error: false, message: "Message successfully deleted".to_owned() }),
                Err(_) => Err(GenericError { error: true, message: "An error ocurred deleting the message".to_owned() })
            }
        },
        Err(err) => Err(err)
    }
}