extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;
use crate::utilities::achievements::*;

pub fn create_column(column_info: &ColumnInputCreate, user_id: &str) -> Result<Columna, GenericError> {
    let connection = &mut establish_connection();
    let column_id = uuid::Uuid::new_v4().to_string();
    let new_column = Columna {
        id: column_id.clone(),
        idboard: column_info.idboard.clone(),
        title: column_info.title.clone()
    };
    let created_column = diesel::insert_into(columna::table)
        .values(&new_column)
        .get_result::<Columna>(connection);

    match created_column {
        Ok(column) => {
            let achievement_updated = check_update_user_achievement(user_id, "6");
            match achievement_updated {
                Ok(_) => Ok(column),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}

pub fn update_column(column_info: &ColumnInputCreate, column_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let column_found: Result<Columna, Error> = columna::table.filter(columna::id.eq(column_id)).first::<Columna>(connection);
    match column_found {
        Ok(mut column) => {
            column.title = column_info.title.clone();

            let updated_project = column.save_changes::<Columna>(connection);
            match updated_project {
                Ok(_user) => Ok(GenericError {error: false, message: "Updated column successfully".to_string()}), 
                Err(err) => Err(GenericError {error: true, message: err.to_string()})
            }
        },
        Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
    }
}

pub fn delete_column(column_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let column_found: Result<Columna, Error> = columna::table.filter(columna::id.eq(column_id)).first::<Columna>(connection);
    match column_found {
        Ok(column) => {
            let deleted = diesel::delete(columna::table.filter(columna::id.eq(column.id))).execute(connection);
            match deleted {
                Ok(_) => Ok(GenericError { error: false, message: "Column deleted successfully".to_string() }),
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
    }
}