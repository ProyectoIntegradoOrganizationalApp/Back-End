use crate::models::models::*;
use crate::schema::*;
use chrono::Utc;
use diesel::prelude::*;
use rust_api_rest::establish_connection;
use serde_json::Value;
use crate::utilities::user as user_utils;
use crate::utilities::chat as chat_utils;

pub fn create_message(user_id: &String, message_data: &MessageInput, connection: &mut PgConnection) -> Result<Message, GenericError> {
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

pub fn update_message(message_id: &String, user_id: &String, message_data: &MessageUpdate, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
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

pub fn delete_message(message_id: &String, user_id: &String, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
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

pub fn get_group(group_id: &String, connection: &mut PgConnection) -> Result<Group, GenericError> {
    let group_found = groups::table.filter(groups::id.eq(group_id)).first::<Group>(connection);
    match group_found {
        Ok(group) => Ok(group),
        Err(_) => Err(GenericError { error: true, message: "Couldn't find a group with that id".to_owned() })
    }
}

pub fn group_exists(group_id: &String, connection: &mut PgConnection) -> bool {
    let group_found = groups::table.filter(groups::id.eq(group_id)).first::<Group>(connection);
    match group_found {
        Ok(_) => true,
        Err(_) => false
    }
}

#[allow(unused)]
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

pub fn handle_websocket_message(message: Value) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let converted_message: Result<WSMessage, serde_json::Error> = serde_json::from_value(message);

    match converted_message {
        Ok(ws_message) => {
            match ws_message.action.as_str() {
                "create" => {
                    if ws_message.data_create.is_some() {
                        match create_message(&ws_message.iduser, &ws_message.data_create.unwrap(), connection) {
                            Ok(_) => Ok(GenericError { error: false, message: "The message was successfully created".to_owned() }),
                            Err(err) => Err(err)
                        }
                    } else {
                        Err(GenericError { error: true, message: "You must provide data to create a message".to_owned() })
                    }
                },
                "update" => {
                    if ws_message.data_update.is_some() && ws_message.idmessage.is_some() {
                        match update_message(&ws_message.idmessage.unwrap(), &ws_message.iduser, & ws_message.data_update.unwrap(), connection) {
                            Ok(_) => Ok(GenericError { error: false, message: "The message was successfully updated".to_owned() }),
                            Err(err) => Err(err)
                        }
                    } else {
                        Err(GenericError { error: true, message: "You must provide the data and id of the message to update".to_owned() })
                    }
                },
                "delete" => {
                    if ws_message.idmessage.is_some() {
                        match delete_message(&ws_message.idmessage.unwrap(), &ws_message.iduser, connection) {
                            Ok(_) => Ok(GenericError { error: false, message: "The message was successfully deleted".to_owned() }),
                            Err(err) => Err(err)
                        }
                    } else {
                        Err(GenericError { error: true, message: "You must provide the id of the message you want to delete".to_owned() })
                    }

                },
                _ => return Err(GenericError { error: true, message: "The action must be a valid value".to_owned() })
            }
        },
        Err(_) => Err(GenericError { error: true, message: "Message format error".to_owned() })
    }
    
}