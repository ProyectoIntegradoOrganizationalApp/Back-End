extern crate redis;

use crate::models::models::*;
use crate::schema::projects;
use diesel::prelude::*;
use rust_api_rest::establish_connection;

use chrono::Utc;

pub fn create_project(project_info: &ProjectInputCreate, token_iduser: String) -> Result<Project, GenericError> {
    let connection = &mut establish_connection();
    let project_id = uuid::Uuid::new_v4().to_string();
    let now: String = (Utc::now()).to_string();
    let new_project = Project {
        idproject: project_id,
        iduser: token_iduser,
        name: project_info.name.clone(),
        description: project_info.description.clone(),
        created_at: now.clone(),
        updated_at: now.clone()
    };
    let created_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result::<Project>(connection);

    match created_project {
        Ok(project) => Ok(project),
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}