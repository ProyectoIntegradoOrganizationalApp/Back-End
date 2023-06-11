extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use chrono::Utc;
use diesel::prelude::*;

pub fn create_default_board(app_id: &String, project_id: &String, user_id: &String, connection: &mut PgConnection) -> Result<Board, GenericError> {
    let board_id = uuid::Uuid::new_v4().to_string();
    let now: String = (Utc::now()).to_string();
    let new_board = Board {
        id: board_id.clone(),
        idapp: app_id.clone(),
        iduser: user_id.clone(),
        idproject: project_id.clone(),
        title: "Board-Default".to_string(),
        photo: "https://www.tooltyp.com/wp-content/uploads/2014/10/1900x920-8-beneficios-de-usar-imagenes-en-nuestros-sitios-web.jpg".to_string(),
        created_at: now.clone(),
        updated_at: now.clone()
    };
    let created_board = diesel::insert_into(board::table)
        .values(&new_board)
        .get_result::<Board>(connection);

    match created_board {
        Ok(board) => Ok(board),
        Err(_) => Err(GenericError { error: false, message: "Error while creating the board".to_string() })
    }
}