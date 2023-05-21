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
                idproject: idproject.clone()
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
                        "kanban" => {
                            if app_info.kanban.is_some() {
                                match app_utils::create_app_by_type(&app, app_info, AppTypes::Kanban(kanban::table), connection) {
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
                                return Err(GenericError { error: true, message: "You need to provide the required info to create a kanban app".to_owned() })
                            }
                        },
                        "docs" => {
                            if app_info.docs.is_some() {
                                match app_utils::create_app_by_type(&app, app_info, AppTypes::Docs(docs::table), connection) {
                                    Ok(_) => return Ok(app),
                                    Err(err) => return Err(err)
                                }
                            } else {
                                return Err(GenericError { error: true, message: "You need to provide the required info to create a docs app".to_owned() })
                            }
                        },
                        "timeline" => {
                            if app_info.timeline.is_some() {
                                match app_utils::create_app_by_type(&app, app_info, AppTypes::Timeline(timeline::table), connection) {
                                    Ok(_) => {
                                        let achievement_updated = achievements::check_update_user_achievement(token_iduser, "4");
                                        match achievement_updated {
                                            Ok(_) => return Ok(app),
                                            Err(err) => return Err(err)
                                        }
                                    },
                                    Err(err) => return Err(err)
                                }
                            } else {
                                return Err(GenericError { error: true, message: "You need to provide the required info to create a timeline app".to_owned() })
                            }
                        },
                        _ => Err(GenericError { error: true, message: "The app type must be specified".to_owned() })
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
                "kanban" => {
                    match app_utils::update_app_by_type(project_id, app_id, app_info, AppTypes::Kanban(kanban::table)) {
                        Ok(response) => return Ok(response),
                        Err(err) => return Err(err)
                    }
                },
                "docs" => {
                    match app_utils::update_app_by_type(project_id, app_id, app_info, AppTypes::Docs(docs::table)) {
                        Ok(response) => return Ok(response),
                        Err(err) => return Err(err)
                    }
                },
                "timeline" => {
                    match app_utils::update_app_by_type(project_id, app_id, app_info, AppTypes::Timeline(timeline::table)) {
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