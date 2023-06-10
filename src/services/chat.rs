use crate::models::models::*;
use crate::schema::*;
use chrono::Utc;
use diesel::prelude::*;
use rust_api_rest::establish_connection;
use crate::utilities::chat as chat_utils;
use crate::middlewares::middlewares::ws_request_validation;
use rocket::{serde::json::{to_string as json_to_string}};
use async_std::{net::TcpStream};
use serde_json::{Value};
use async_tungstenite::{tungstenite::protocol::{Message as WSMessage}, accept_hdr_async};
// use rocket::{futures::{StreamExt, SinkExt, TryStreamExt}};

use futures::prelude::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut,
};
use crate::rocket::futures::{StreamExt, TryStreamExt, SinkExt};

pub async fn websocket_handler(stream: TcpStream) {
    let ws_validation = accept_hdr_async(stream, ws_request_validation).await;

    match ws_validation {
        Ok(ws) => {
            
            let (mut write, mut read) = ws.split();

            write.send(WSMessage::Text("HOLA WENAS".to_owned()));

            // let broadcast_incoming = read.next().await;
                
            // match broadcast_incoming {
            //     Some(message_result) => {
            //         match message_result {
            //             Ok(message) => {
            //                 println!("message");
            //             },
            //             Err(err) => println!("{}", err)
            //         }
            //     },
            //     None => println!("NADADADA")
            // }

            while let Some( msg ) = read.next().await
            {
               let msg = match msg
               {
                  Err(e) => // <----------------- Aquí
                  {
                     error!( "Error on server stream: {:?}", e.to_string() ); 
         
                     // Errors returned directly through the AsyncRead/Write API are fatal, generally an error on the underlying
                     // transport.
                     //
                     continue;
                  }
         
                  Ok(m) => m,
               };
         
         
               println!("Recibido: {}", msg.to_string());
         
               // ... do something useful
            }
            
            // while let Some(Ok(msg)) = read.next().await {
            //     // Procesar los mensajes recibidos
            //     match msg {
            //         WSMessage::Text(text) => {
            //             let is_json: Result<Value, serde_json::Error> = serde_json::from_str(text.as_str());
            //             match is_json {
            //                 Ok(value) => {
            //                     match chat_utils::handle_websocket_message(value) {
            //                         Ok(response) => {
            //                             match write.send(WSMessage::Text(json_to_string(&response).unwrap())).await {
            //                                 Ok(_) => {},
            //                                 Err(err) => println!("{}", err.to_string())
            //                             }
            //                         },
            //                         Err(err) => {
            //                             match write.send(WSMessage::Text(json_to_string(&err).unwrap())).await {
            //                                 Ok(_) => {},
            //                                 Err(error) => println!("{}", error.to_string())
            //                             }
            //                         }
            //                     }
            //                 },
            //                 Err(_) => {
            //                     let response = GenericError { error: true, message: "Message format error".to_owned() };
            //                     match write.send(WSMessage::Text(json_to_string(&response).unwrap())).await {
            //                         Ok(_) => {},
            //                         Err(err) => println!("{}", err.to_string())
            //                     }
            //                 }
            //             }
            //         }
            //         WSMessage::Close(_) => {
            //             // Manejar cierre de conexión
            //             break;
            //         }
            //         _ => {}
            //     }
            // }
        },
        Err(err) => {println!("{}", err.to_string())}
    }
}

// GROUP CRUD -------------------
pub fn create_group(group_info: &GroupInputCreate, user_id: &String) -> Result<Group, GenericError> {
    let connection = &mut establish_connection();
    let new_group = Group {
        id:  uuid::Uuid::new_v4().to_string(),
        iduser: user_id.clone(),
        title: group_info.title.clone()
    };
    let group_created = diesel::insert_into(groups::table)
        .values(&new_group)
        .get_result::<Group>(connection);

    match group_created {
        Ok(group) => {
            let new_group_user = GroupUser {
                iduser: group.iduser.clone(),
                idgroup: group.id.clone(),
                joined_at: Utc::now().naive_utc()
            };
            let user_group_created = diesel::insert_into(group_user::table)
                .values(&new_group_user)
                .get_result::<GroupUser>(connection);
            match user_group_created {
                Ok(_) => Ok(group),
                Err(_) => {
                    let deleted_group = diesel::delete(groups::table.filter(groups::id.eq(group.id))).execute(connection);
                    match deleted_group {
                        _ => Err(GenericError { error: true, message: "An error ocurred adding the user to the group".to_owned() })
                    }
                }
            }
        }
        Err(_) => Err(GenericError { error: true, message: "An error ocurred creating the group".to_owned() })
    }
}

pub fn update_group(group_id: &String, group_info: &GroupInputCreate, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match chat_utils::get_group(group_id, connection) {
        Ok(mut group) => {
            if group.iduser.as_str() == user_id.as_str() {
                group.title = group_info.title.clone();
                let updated_group = group.save_changes::<Group>(connection);
                match updated_group {
                    Ok(_) => Ok(GenericError {error: false, message: "The group was successfully updated".to_string()}), 
                    Err(_) => Err(GenericError {error: true, message: "An error ocurred updating the group".to_owned() })
                }
            } else {
                Err(GenericError {error: true, message: "You must be the owner of the group in order to update it".to_owned() })
            }
        }
        Err(err) => Err(err)
    }
}

pub fn delete_group(group_id: &String, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match chat_utils::get_group(group_id, connection) {
        Ok(group) => {
            if group.iduser.as_str() == user_id.as_str() {
                let deleted_group = diesel::delete(groups::table.filter(groups::id.eq(&group.id))).execute(connection);
                match deleted_group {
                    Ok(_) => {
                        let deleted_group_users = diesel::delete(group_user::table.filter(group_user::idgroup.eq(&group.id))).execute(connection);
                        match deleted_group_users {
                            Ok(_) => Ok(GenericError { error: false, message: "The group was successfully deleted".to_owned() }),
                            Err(_) => Err(GenericError {error: true, message: "An error ocurred deleting the group users".to_owned() })
                        }
                    }, 
                    Err(_) => Err(GenericError {error: true, message: "An error ocurred deleting the group".to_owned() })
                }
            } else {
                Err(GenericError {error: true, message: "You must be the owner of the group in order to delete it".to_owned() })
            }
        }
        Err(err) => Err(err)
    }
}