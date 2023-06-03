extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use crate::utilities::achievements as achievement_utils;

fn create_app(project_id: &str, app_info: &AppInputCreate, connection: &mut PgConnection) -> Result<App, GenericError> {
    let app_id = uuid::Uuid::new_v4().to_string();
    let new_app = App {
        id: app_id.clone(),
        idproject: project_id.to_owned().clone(),
        name: app_info.name.clone(),
        description: app_info.description.clone(),
        photo: app_info.photo.clone()
    };
    let created_app = diesel::insert_into(app::table)
        .values(&new_app)
        .get_result::<App>(connection);

    match created_app {
        Ok(app) => Ok(app),
        Err(_) => Err(GenericError {error: true, message: "An error ocurred trying to create the app".to_owned()})
    }
}

fn update_app(app_id: &str, app_info: &AppInputCreate, connection: &mut PgConnection) -> Result<(), GenericError>{
    let found_app = app::table.filter(app::id.eq(app_id))
        .first::<App>(connection);

    match found_app {
        Ok(mut app) => {
            app.name = app_info.name.clone();
            app.description = app_info.description.clone();
            app.photo = app_info.photo.clone();
            let updated_app = app.save_changes::<App>(connection);
            match updated_app {
                Ok(_) => Ok(()), 
                Err(_) => Err(GenericError {error: true, message: "An error ocurred trying to update the app".to_owned()})
            }
        },
        Err(_) => Err(GenericError {error: true, message: "Couldn't find an app with that id".to_owned()})
    }
}

pub fn create_app_by_type(project_id: &str, user_id: &str, app_info: &AppInputCreate, connection: &mut PgConnection) -> Result<App, GenericError> {
    match create_app(project_id, app_info, connection) {
        Ok(app) => {
            match app_info.apptype.as_str() {
                "task_app" => {
                    let task_app_info = app_info.task_app.as_ref().unwrap();
                    let new_task_app = TaskApp {
                        idapp: app.id.clone(),
                        idproject: app.idproject.clone(),
                        app_type: task_app_info.app_type.clone()
                    };
                    let created_app_type = diesel::insert_into(task_app::table)
                        .values(&new_task_app)
                        .get_result::<TaskApp>(connection);
                    match created_app_type {
                        Ok(_) => { 
                            let achievement_updated = achievement_utils::check_update_user_achievement(user_id, "3");
                            match achievement_updated {
                                Ok(_) => Ok(app),
                                Err(err) => return Err(err)
                            }
                        },
                        Err(err) => Err(GenericError { error:true, message: err.to_string()})
                    }
                }
                "docs_app" => {
                    let docs_app_info = app_info.docs_app.as_ref().unwrap();
                    let new_docs_app = DocsApp {
                        idapp: app.id.clone(),
                        idproject: app.idproject.clone(),
                        app_type: docs_app_info.app_type.clone()
                    };
                    let created_app_type = diesel::insert_into(docs_app::table)
                        .values(&new_docs_app)
                        .get_result::<DocsApp>(connection);
                    match created_app_type {
                        Ok(_) => Ok(app),
                        Err(err) => Err(GenericError { error:true, message: err.to_string()})
                    }
                },
                _ => Err(GenericError { error: true, message: "You need to provide a valid app type".to_owned() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn update_app_by_type(project_id: &str, app_id: &str, app_info: &AppInputCreate, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
    match update_app(app_id, app_info, connection) {
        Ok(_) => {
            match app_info.apptype.as_str() {
                "task_app" => {
                    let found_app = task_app::table.filter(task_app::idapp.eq(app_id))
                            .filter(task_app::idproject.eq(project_id))
                            .first::<TaskApp>(connection);
                    match found_app {
                        Ok(mut _app) => {
                            // --------- Cuando haya propiedaddes cambiar ---------
                            // let updated_app = app.save_changes::<App>(connection);
                            // match updated_app {
                            //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                            //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                            // }
                            Ok(GenericError {error: false, message: "Updated task app successfully".to_string()})
                        },
                        Err(_) => Err(GenericError { error: true, message: "The task app was not found".to_owned() })
                    }
                },
                "docs_app" => {
                    let found_app = docs_app::table.filter(docs_app::idapp.eq(app_id))
                            .filter(docs_app::idproject.eq(project_id))
                            .first::<DocsApp>(connection);
                    match found_app {
                        Ok(mut _app) => {
                            // --------- Cuando haya propiedaddes cambiar ---------
                            // let updated_app = app.save_changes::<App>(connection);
                            // match updated_app {
                            //     Ok(_) => Ok(GenericError {error: false, message: "Updated kanban app successfully".to_string()}), 
                            //     Err(err) => Err(GenericError {error: true, message: err.to_string()})
                            // }
                            Ok(GenericError {error: false, message: "Updated docs app successfully".to_string()})
                        },
                        Err(_) => Err(GenericError { error: true, message: "The docs app was not found".to_owned() })
                    }
                },
                _ => Err(GenericError { error: true, message: "You need to provide a valid app type".to_owned() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn check_app_by_type(app_id: &str, app_type: &str, connection: &mut PgConnection) -> Result<String, GenericError> {
    let found_app = app::table.filter(app::id.eq(app_id)).first::<App>(connection);
    match found_app {
        Ok(_) => {
            match app_type {
                "task_app" => {
                    let found_app = task_app::table.filter(task_app::idapp.eq(app_id)).first::<TaskApp>(connection);
                    match found_app {
                        Ok(app) => Ok(app.idproject),
                        Err(_) => Err(GenericError { error: true, message: "Couldn't find a task app with that id".to_owned() })
                    }
                },
                "docs_app" => {
                    let found_app = docs_app::table.filter(docs_app::idapp.eq(app_id)).first::<DocsApp>(connection);
                    match found_app {
                        Ok(app) => Ok(app.idproject),
                        Err(_) => Err(GenericError { error: true, message: "Couldn't find a docs app with that id".to_owned() })
                    }
                },
                _ => Err(GenericError { error: true, message: "You need to provide a valid app type".to_owned() })
            }
        },
        Err(_) => Err(GenericError { error: true, message: "Couldn't find an app with that id".to_owned() })
    }
}