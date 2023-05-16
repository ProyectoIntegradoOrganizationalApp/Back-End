extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;

pub fn create_board(board_info: &BoardInputCreate) -> Result<Board, GenericError> {
    let connection = &mut establish_connection();
    let board_id = uuid::Uuid::new_v4().to_string();
    let new_board = Board {
        id: board_id.clone(),
        idapp: board_info.idapp.clone(),
        title: board_info.title.clone()
    };
    let created_board = diesel::insert_into(board::table)
        .values(&new_board)
        .get_result::<Board>(connection);

    match created_board {
        Ok(board) => Ok(board),
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}

pub fn update_board(board_info: &BoardInputCreate, board_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let board_found: Result<Board, Error> = board::table.filter(board::id.eq(board_id)).first::<Board>(connection);
    match board_found {
        Ok(mut board) => {
            board.title = board_info.title.clone();

            let updated_project = board.save_changes::<Board>(connection);
            match updated_project {
                Ok(_user) => Ok(GenericError {error: false, message: "Updated Board successfully".to_string()}), 
                Err(err) => Err(GenericError {error: true, message: err.to_string()})
            }
        },
        Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
    }
}

pub fn delete_board(board_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
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
}