extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;

use chrono::Utc;

pub fn create_project(project_info: &ProjectInputCreate, token_iduser: String) -> Result<Project, GenericError> {
    let connection = &mut establish_connection();
    let project_id = uuid::Uuid::new_v4().to_string();
    let now: String = (Utc::now()).to_string();
    let new_project = Project {
        idproject: project_id.clone(),
        iduser: token_iduser.clone(),
        name: project_info.name.clone(),
        description: project_info.description.clone(),
        created_at: now.clone(),
        updated_at: now.clone()
    };
    let created_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result::<Project>(connection);

    match created_project {
        Ok(project) => {
            let new_project_user = UserProject {
                idproject: project_id.clone(),
                iduser: token_iduser.clone(),
                idrole: "1".to_string()
            };
            let created_project_user = diesel::insert_into(project_user::table)
                .values(&new_project_user)
                .get_result::<UserProject>(connection);
            match created_project_user {
                Ok(_user_project) => Ok(project),
                Err(err) => Err(GenericError { error: false, message: err.to_string() })
            }
        },
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}

pub fn update_project(project_info: &ProjectInputCreate, user_id: &String, project_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let user_found = users::table.filter(users::id.eq(&user_id)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let project_found: Result<Project, Error> = UserProject::belonging_to(&user)
                            .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                            .filter(projects::idproject.eq(project_id))
                            .filter(project_user::idrole.eq("1".to_string()))
                            .select(Project::as_select())
                            .get_result::<Project>(connection);
            
            match project_found {
                Ok(mut project) => {
                    project.name = project_info.name.clone();
                    project.description = project_info.description.clone();
                    project.updated_at = (Utc::now()).to_string();

                    let updated_project = project.save_changes::<Project>(connection);
                    match updated_project {
                        Ok(_user) => Ok(GenericError {error: false, message: "Updated project successfully".to_string()}), 
                        Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    }
                },
                Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
            }
        }, 
        Err(err) => Err(GenericError {error: true, message: err.to_string()})
    }
}

pub fn delete_project(user_id: &String, project_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let user_found = users::table.filter(users::id.eq(&user_id)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let project_found: Result<Project, Error> = UserProject::belonging_to(&user)
                            .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                            .filter(projects::idproject.eq(project_id))
                            .filter(project_user::idrole.eq("1".to_string()))
                            .select(Project::as_select())
                            .get_result::<Project>(connection);
            
            match project_found {
                Ok(project) => {
                    let deleted = diesel::delete(projects::table.filter(projects::idproject.eq(project.idproject))).execute(connection);
                    match deleted {
                        Ok(_) => Ok(GenericError { error: false, message: "Project deleted successfully".to_string() }),
                        Err(err) => Err(GenericError { error: true, message: err.to_string() })
                    }
                },
                Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
            }
        }, 
        Err(err) => Err(GenericError {error: true, message: err.to_string()})
    }
}