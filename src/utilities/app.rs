extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rust_api_rest::establish_connection;

pub fn create_app_by_type(app: &App, app_info: &AppInputCreate, table: AppTypes, connection: &mut PgConnection) -> Result<(), GenericError> {
    match table {
        AppTypes::TaskApp(task_app_table) => {
            let task_app_info = app_info.task_app.as_ref().unwrap();
            let new_task_app = TaskApp {
                idapp: app.id.clone(),
                idproject: app.idproject.clone(),
                app_type: task_app_info.app_type.clone()
            };
            let created_app_type = diesel::insert_into(task_app_table)
                .values(&new_task_app)
                .get_result::<TaskApp>(connection);
            match created_app_type {
                Ok(_) => Ok(()),
                Err(err) => Err(GenericError { error:true, message: err.to_string()})
            }
        }
        AppTypes::DocsApp(docs_app_table) => {
            let docs_app_info = app_info.docs_app.as_ref().unwrap();
            let new_docs_app = DocsApp {
                idapp: app.id.clone(),
                idproject: app.idproject.clone(),
                app_type: docs_app_info.app_type.clone()
            };
            let created_app_type = diesel::insert_into(docs_app_table)
                .values(&new_docs_app)
                .get_result::<DocsApp>(connection);
            match created_app_type {
                Ok(_) => Ok(()),
                Err(err) => Err(GenericError { error:true, message: err.to_string()})
            }
        }
    }
}

pub fn update_app_by_type(project_id: &str, app_id: &str, app_info: &AppInputCreate, table: AppTypes) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match table {
        AppTypes::TaskApp(task_app_table) => {
            let found_app = task_app_table.filter(task_app::idapp.eq(app_id))
                    .filter(task_app::idproject.eq(project_id))
                    .first::<TaskApp>(connection);
            match found_app {
                Ok(mut app) => {
                    // --------- Cuando haya propiedaddes cambiar ---------
                    // let updated_app = app.save_changes::<App>(connection);
                    // match updated_app {
                    //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                    //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    // }
                    Ok(GenericError {error: false, message: "Updated task app successfully".to_string()})
                },
                Err(err) => Err(GenericError { error: true, message: "The task app was not found".to_owned() })
            }
        },
        AppTypes::DocsApp(docs_app_table) => {
            let found_app = docs_app_table.filter(docs_app::idapp.eq(app_id))
                    .filter(docs_app::idproject.eq(project_id))
                    .first::<DocsApp>(connection);
            match found_app {
                Ok(mut app) => {
                    // --------- Cuando haya propiedaddes cambiar ---------
                    // let updated_app = app.save_changes::<App>(connection);
                    // match updated_app {
                    //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                    //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    // }
                    Ok(GenericError {error: false, message: "Updated docs app successfully".to_string()})
                },
                Err(err) => Err(GenericError { error: true, message: "The docs app was not found".to_owned() })
            }
        }
    }
}