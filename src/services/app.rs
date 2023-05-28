extern crate redis;

use crate::models::models::*;
use crate::utilities::project;
use crate::utilities::app as app_utils;
use crate::utilities::achievements;
use crate::schema::*;
use diesel::prelude::*;
use rust_api_rest::establish_connection;

pub fn create_app(app_info: &AppInputCreate, idproject: &String, token_iduser: &String) -> Result<App, GenericError> {
    match project::is_admin(&idproject, &token_iduser) {
        Ok(_) => {
            let connection = &mut establish_connection();

            let app_id = uuid::Uuid::new_v4().to_string();
            let new_app = App {
                id: app_id.clone(),
                idproject: idproject.clone(),
                name: app_info.name.clone(),
                description: app_info.description.clone(),
                photo: app_info.photo.clone()
            };
            let created_app = diesel::insert_into(app::table)
                .values(&new_app)
                .get_result::<App>(connection);

            match created_app {
                Ok(app) => {
                    let achievement_updated = achievements::check_update_user_achievement(token_iduser, "7");
                    match achievement_updated {
                        Ok(_) => {},
                        Err(err) => return Err(err)
                    }
                    match app_info.apptype.as_str() {
                        "task_app" => {
                            if app_info.task_app.is_some() {
                                match app_utils::create_app_by_type(&app, app_info, AppTypes::TaskApp(task_app::table), connection) {
                                    Ok(_) => {
                                        let achievement_updated = achievements::check_update_user_achievement(token_iduser, "3");
                                        match achievement_updated {
                                            Ok(_) => return Ok(app),
                                            Err(err) => return Err(err)
                                        }
                                    },
                                    Err(err) => return Err(err)
                                }
                            } else {
                                return Err(GenericError { error: true, message: "You need to provide the required info to create a task type app".to_owned() })
                            }
                        },
                        "docs_app" => {
                            if app_info.docs_app.is_some() {
                                match app_utils::create_app_by_type(&app, app_info, AppTypes::DocsApp(docs_app::table), connection) {
                                    Ok(_) => return Ok(app),
                                    Err(err) => return Err(err)
                                }
                            } else {
                                return Err(GenericError { error: true, message: "You need to provide the required info to create a docs type app".to_owned() })
                            }
                        },
                        _ => {
                            let deleted = diesel::delete(app::table.filter(app::id.eq(app.id))).execute(connection);
                            match deleted {
                                Ok(_) => Err(GenericError { error: true, message: "The app type must be valid".to_owned() }),
                                Err(_) => Err(GenericError { error: true, message: "An error ocurred while creating the app".to_owned() }),
                            }
                            
                        }
                    }
                },
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn update_app(app_info: &AppInputCreate, user_id: &String, project_id: &String, app_id: &String) -> Result<GenericError, GenericError> {
    match project::is_admin(&project_id, &user_id) {
        Ok(_) => {
            match app_info.apptype.as_str() {
                "task_app" => {
                    match app_utils::update_app_by_type(project_id, app_id, app_info, AppTypes::TaskApp(task_app::table)) {
                        Ok(response) => return Ok(response),
                        Err(err) => return Err(err)
                    }
                },
                "docs_app" => {
                    match app_utils::update_app_by_type(project_id, app_id, app_info, AppTypes::DocsApp(docs_app::table)) {
                        Ok(response) => return Ok(response),
                        Err(err) => return Err(err)
                    }
                },
                _ => Err(GenericError { error: true, message: "The app type must be specified".to_owned() })
            }
        },
        Err(err) => return Err(err)
    }
}

pub fn delete_app(app_id: &String, project_id: &String, user_id: &String) -> Result<GenericError, GenericError> {
    match project::is_admin(&project_id, &user_id) {
        Ok(_) => {
            let connection = &mut establish_connection();
            let app_found = app::table.filter(app::id.eq(&app_id))
                        .filter(app::idproject.eq(project_id))
                        .first::<App>(connection);
            match app_found {
                Ok(_) => {
                    let deleted = diesel::delete(app::table)
                                .filter(app::id.eq(app_id))
                                .filter(app::idproject.eq(project_id))
                                .execute(connection);
                    match deleted {
                        Ok(_) => Err(GenericError { error: true, message: "The app was successfully deleted".to_owned() }),
                        Err(err) => Err(GenericError { error: true, message: err.to_string() })
                    }
                }, 
                Err(_) => Err(GenericError { error: true, message: "The app was not found".to_owned() })
            }
        },
        Err(err) => Err(err)
    }
}