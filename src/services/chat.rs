use crate::models::models::*;
use crate::utilities::chat as chat_utils;
use crate::middlewares::middlewares::ws_request_validation;
use rocket::{serde::json::{to_string as json_to_string}};
use tokio::net::TcpStream;
use serde_json::{Value};
use tokio_tungstenite::{tungstenite::Message as WSMessage, accept_hdr_async};
use rocket::{futures::{StreamExt, SinkExt}};

pub async fn websocket_handler(stream: TcpStream) {
    let ws_validation = accept_hdr_async(stream, ws_request_validation).await;

    match ws_validation {
        Ok(ws) => {
            let (mut write, mut read) = ws.split();
            
            // Aquí puedes implementar la lógica específica para tu servidor WebSocket
            while let Some(Ok(msg)) = read.next().await {
                // Procesar los mensajes recibidos
                match msg {
                    WSMessage::Text(text) => {
                        // Manejar mensaje de texto
                        
                        let is_json: Result<Value, serde_json::Error> = serde_json::from_str(text.as_str());
                        match is_json {
                            Ok(value) => {
                                match chat_utils::handle_websocket_message(value) {
                                    Ok(response) => {
                                        match write.send(WSMessage::Text(json_to_string(&response).unwrap())).await {
                                            Ok(_) => {},
                                            Err(err) => println!("{}", err.to_string())
                                        }
                                    },
                                    Err(err) => {
                                        match write.send(WSMessage::Text(json_to_string(&err).unwrap())).await {
                                            Ok(_) => {},
                                            Err(error) => println!("{}", error.to_string())
                                        }
                                    }
                                }
                            },
                            Err(_) => {
                                let response = GenericError { error: true, message: "Message format error".to_owned() };
                                match write.send(WSMessage::Text(json_to_string(&response).unwrap())).await {
                                    Ok(_) => {},
                                    Err(err) => println!("{}", err.to_string())
                                }
                            }
                        }
                    }
                    WSMessage::Close(_) => {
                        // Manejar cierre de conexión
                        break;
                    }
                    _ => {}
                }
            }
        },
        Err(err) => {println!("{}", err.to_string())}
    }
}