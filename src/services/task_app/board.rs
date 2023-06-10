extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;
use crate::utilities::user as user_utils;
use crate::utilities::app as app_utils;
use chrono::Utc;

pub fn create_board(id_app: &str, board_info: &BoardInputCreate, user_id: &str) -> Result<Board, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let board_id = uuid::Uuid::new_v4().to_string();
                let now: String = (Utc::now()).to_string();
                let new_board = Board {
                    id: board_id.clone(),
                    idapp: id_app.to_owned(),
                    iduser: user_id.to_owned().clone(),
                    idproject: project_id.clone(),
                    title: board_info.title.clone(),
                    photo: board_info.photo.clone(),
                    created_at: now.clone(),
                    updated_at: now.clone()
                };
                let created_board = diesel::insert_into(board::table)
                    .values(&new_board)
                    .get_result::<Board>(connection);
            
                match created_board {
                    Ok(board) => Ok(board),
                    Err(err) => Err(GenericError { error: false, message: err.to_string() })
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn update_board(id_app: &str, board_info: &BoardInputCreate, board_id: &String, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let board_found: Result<Board, Error> = board::table.filter(board::id.eq(board_id)).first::<Board>(connection);
                match board_found {
                    Ok(mut board) => {
                        let now: String = (Utc::now()).to_string();
                        board.title = board_info.title.clone();
                        board.photo = board_info.photo.clone();
                        board.updated_at = now.clone();
            
                        let updated_project = board.save_changes::<Board>(connection);
                        match updated_project {
                            Ok(_user) => Ok(GenericError {error: false, message: "Updated Board successfully".to_string()}), 
                            Err(err) => Err(GenericError {error: true, message: err.to_string()})
                        }
                    },
                    Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to do that".to_string()})
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn delete_board(id_app: &str, board_id: &String, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let board_found: Result<Board, Error> = board::table.filter(board::id.eq(board_id)).first::<Board>(connection);
                match board_found {
                    Ok(board) => {
                        let deleted = diesel::delete(board::table.filter(board::id.eq(board.id))).execute(connection);
                        match deleted {
                            Ok(_) => Ok(GenericError { error: false, message: "Board deleted successfully".to_string() }),
                            Err(err) => Err(GenericError { error: true, message: err.to_string() })
                        }
                    },
                    Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn get_boards(id_app: &str, user_id: &str) -> Result<Vec<Board>, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_member(&project_id, user_id, connection) {
                let found_boards = board::table.filter(board::idapp.eq(id_app)).load::<Board>(connection);
                match found_boards {
                    Ok(boards) => Ok(boards),
                    Err(_) => Err(GenericError { error: true, message: "An error ocurred while getting the boards".to_owned() })
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of the project".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}