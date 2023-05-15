extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;

pub fn create_task(task_info: &TaskInputCreate) -> Result<Task, GenericError> {
    let connection = &mut establish_connection();
    let task_id = uuid::Uuid::new_v4().to_string();
    let new_task = Task {
        id: task_id.clone(),
        idcolumn: task_info.idcolumn.clone(),
        title: task_info.title.clone(),
        description: task_info.description.clone(),
        github: task_info.github.clone()
    };
    let created_task = diesel::insert_into(task::table)
        .values(&new_task)
        .get_result::<Task>(connection);

    match created_task {
        Ok(task) => Ok(task),
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}

pub fn update_task(task_info: &TaskInputCreate, task_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let task_found: Result<Task, Error> = task::table.filter(task::id.eq(task_id)).first::<Task>(connection);
    match task_found {
        Ok(mut task) => {
            task.title = task_info.title.clone();
            task.description = task_info.description.clone();
            task.github = task_info.github.clone();

            let updated_project = task.save_changes::<Task>(connection);
            match updated_project {
                Ok(_user) => Ok(GenericError {error: false, message: "Updated task successfully".to_string()}), 
                Err(err) => Err(GenericError {error: true, message: err.to_string()})
            }
        },
        Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
    }
}

pub fn delete_task(task_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let task_found: Result<Task, Error> = task::table.filter(task::id.eq(task_id)).first::<Task>(connection);
    match task_found {
        Ok(task) => {
            let deleted = diesel::delete(task::table.filter(task::id.eq(task.id))).execute(connection);
            match deleted {
                Ok(_) => Ok(GenericError { error: false, message: "Task deleted successfully".to_string() }),
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
    }
}