extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;
use crate::utilities::achievements::*;
use crate::utilities::user as user_utils;
use crate::utilities::app as app_utils;
use crate::utilities::task_app as task_app_utils;
use chrono::Utc;

pub fn create_column(column_info: &ColumnInputCreate, id_app: &str, user_id: &str) -> Result<Columna, GenericError> { 
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            match task_app_utils::board::check_board_in_app(&column_info.idboard, &id_app.to_owned(), connection) {
                Ok(_) => {
                    if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                        let order = task_app_utils::column::get_last_column_order(&column_info.idboard, connection);
                        let column_id = uuid::Uuid::new_v4().to_string();
                        let now: String = (Utc::now()).to_string();
                        let new_column = Columna {
                            id: column_id.clone(),
                            idboard: column_info.idboard.clone(),
                            iduser: user_id.to_owned().clone(),
                            idproject: project_id.clone(),
                            title: column_info.title.clone(),
                            ordering: order,
                            created_at: now.clone(),
                            updated_at: now.clone()
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
                            Err(_) => Err(GenericError { error: false, message: "Error creating the column".to_string() })
                        }
                    } else {
                        Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
                    }
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

pub fn update_column(column_info: &ColumnInputUpdate, column_id: &String, id_app: &str, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let column_found: Result<Columna, Error> = columna::table.filter(columna::id.eq(column_id)).first::<Columna>(connection);
                match column_found {
                    Ok(mut column) => {
                        let now: String = (Utc::now()).to_string();
                        column.title = column_info.title.clone();
                        column.ordering = column_info.order;
                        column.updated_at = now.clone();
            
                        let updated_project = column.save_changes::<Columna>(connection);
                        match updated_project {
                            Ok(_user) => Ok(GenericError {error: false, message: "Updated column successfully".to_string()}), 
                            Err(_) => Err(GenericError {error: true, message: "Error updating the column".to_string()})
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

pub fn delete_column(column_id: &String, id_app: &str, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let column_found: Result<Columna, Error> = columna::table.filter(columna::id.eq(column_id)).first::<Columna>(connection);
                match column_found {
                    Ok(column) => {
                        let deleted = diesel::delete(columna::table.filter(columna::id.eq(column.id))).execute(connection);
                        match deleted {
                            Ok(_) => Ok(GenericError { error: false, message: "Column deleted successfully".to_string() }),
                            Err(_) => Err(GenericError { error: true, message: "Error deleting the column".to_string() })
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

pub fn get_columns_tasks(id_app: &str, id_board: &str, user_id: &str) -> Result<Vec<ColumnTasks>, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_member(&project_id, user_id, connection) {
                let found_columns = columna::table.filter(columna::idboard.eq(id_board)).load::<Columna>(connection);
                match found_columns {
                    Ok(columns) => {
                        let column_tasks = task_app_utils::column::get_column_tasks(columns, connection);
                        Ok(column_tasks)
                    },
                    Err(_) => Err(GenericError { error: true, message: "No columns found for the board".to_owned() })
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of the project".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}