extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rust_api_rest::establish_connection;

pub fn create_app_by_type(app: &App, app_info: &AppInputCreate, table: AppTypes, connection: &mut PgConnection) -> Result<(), GenericError> {
    match table {
        AppTypes::Kanban(kanban_table) => {
            let kanban_info = app_info.kanban.as_ref().unwrap(); // Por si el algún momento tiene más campos
            let new_kanban = Kanban {
                idapp: app.id.clone(),
                idproject: app.idproject.clone()
            };
            let created_app_type = diesel::insert_into(kanban_table)
                .values(&new_kanban)
                .get_result::<Kanban>(connection);
            match created_app_type {
                Ok(_) => Ok(()),
                Err(err) => Err(GenericError { error:true, message: err.to_string()})
            }
        }
        AppTypes::Docs(docs_table) => {
            let docs_info = app_info.docs.as_ref().unwrap(); // Por si el algún momento tiene más campos
            let new_docs = Docs {
                idapp: app.id.clone(),
                idproject: app.idproject.clone()
            };
            let created_app_type = diesel::insert_into(docs_table)
                .values(&new_docs)
                .get_result::<Docs>(connection);
            match created_app_type {
                Ok(_) => Ok(()),
                Err(err) => Err(GenericError { error:true, message: err.to_string()})
            }
        }
        AppTypes::Timeline(timeline_table) => {
            let timeline_info = app_info.timeline.as_ref().unwrap(); // Por si el algún momento tiene más campos
            let new_timeline = Timeline {
                idapp: app.id.clone(),
                idproject: app.idproject.clone()
            };
            let created_app_type = diesel::insert_into(timeline_table)
                .values(&new_timeline)
                .get_result::<Timeline>(connection);
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
        AppTypes::Kanban(kanban_table) => {
            let found_app = kanban_table.filter(kanban::idapp.eq(app_id))
                    .filter(kanban::idproject.eq(project_id))
                    .first::<Kanban>(connection);
            match found_app {
                Ok(mut app) => {
                    // --------- Cuando haya propiedaddes cambiar ---------
                    // let updated_app = app.save_changes::<App>(connection);
                    // match updated_app {
                    //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                    //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    // }
                    Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()})
                },
                Err(err) => Err(GenericError { error: true, message: "The kanban app was not found".to_owned() })
            }
        },
        AppTypes::Docs(docs_table) => {
            let found_app = docs_table.filter(docs::idapp.eq(app_id))
                    .filter(docs::idproject.eq(project_id))
                    .first::<Docs>(connection);
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
        },
        AppTypes::Timeline(timeline_table) => {
            let found_app = timeline_table.filter(timeline::idapp.eq(app_id))
                    .filter(timeline::idproject.eq(project_id))
                    .first::<Timeline>(connection);
            match found_app {
                Ok(mut app) => {
                    // --------- Cuando haya propiedaddes cambiar ---------
                    // let updated_app = app.save_changes::<App>(connection);
                    // match updated_app {
                    //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                    //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    // }
                    Ok(GenericError {error: false, message: "Updated timeline app successfully".to_string()})
                },
                Err(err) => Err(GenericError { error: true, message: "The timeline app was not found".to_owned() })
            }
        }
    }
}