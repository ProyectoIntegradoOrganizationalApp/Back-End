extern crate redis;

use crate::models::models::*;
use crate::utilities::user as user_utils;
use crate::utilities::app as app_utils;
use crate::utilities::project as project_utils;
use crate::utilities::achievements;
use crate::schema::*;
use diesel::prelude::*;
use rust_api_rest::establish_connection;

pub fn create_app(app_info: &AppInputCreate, project_id: &String, user_id: &String) -> Result<App, GenericError> {
    let connection = &mut establish_connection();
    match project_utils::get_project(project_id, connection) {
        Ok(_) => {
            match user_utils::is_admin(project_id, &user_id, connection) {
                true => {
                    match app_utils::create_app_by_type(project_id, user_id, app_info, connection) {
                        Ok(app) => {
                            let achievement_updated = achievements::check_update_user_achievement(user_id, "7");
                            match achievement_updated {
                                Ok(_) => {},
                                Err(err) => return Err(err)
                            }
                            return Ok(app)
                        },
                        Err(err) => return Err(err)
                    }
                },
                false => Err(GenericError { error: true, message: "You have to be a member of this project and have the admin role".to_string() })
            }
        },
        Err(err) => Err(GenericError{ error: true, message: err })
    }
}

pub fn update_app(app_info: &AppInputCreate, user_id: &String, project_id: &String, app_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match project_utils::get_project(project_id, connection) {
        Ok(_) => {
            match user_utils::is_admin(&project_id, &user_id, connection) {
                true => {
                    match app_info.apptype.as_str() {
                        "task_app" => {
                            match app_utils::update_app_by_type(project_id, app_id, app_info, connection) {
                                Ok(response) => return Ok(response),
                                Err(err) => return Err(err)
                            }
                        },
                        "docs_app" => {
                            match app_utils::update_app_by_type(project_id, app_id, app_info, connection) {
                                Ok(response) => return Ok(response),
                                Err(err) => return Err(err)
                            }
                        },
                        _ => Err(GenericError { error: true, message: "The app type must be specified".to_owned() })
                    }
                },
                false => Err(GenericError { error: true, message: "You have to be a member of this project and have the admin role".to_string() })
            }
        },
        Err(err) => Err(GenericError{ error: true, message: err })
    }
}

pub fn delete_app(app_id: &String, project_id: &String, user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match project_utils::get_project(project_id, connection) {
        Ok(_) => {
            match user_utils::is_admin(&project_id, &user_id, connection) {
                true => {
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
                false => Err(GenericError { error: true, message: "You have to be a member of this project and have the admin role".to_string() })
            }
        },
        Err(err) => Err(GenericError{ error: true, message: err })
    }
}