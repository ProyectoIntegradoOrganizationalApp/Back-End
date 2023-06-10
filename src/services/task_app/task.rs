extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;
use crate::utilities::achievements::*;
use crate::utilities::user as user_utils;
use crate::utilities::app as app_utils;
use chrono::{Utc, NaiveDateTime, NaiveDate, NaiveTime, Datelike};

pub fn create_task(task_info: &TaskInputCreate, id_app: &str, user_id: &str) -> Result<Task, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let task_id = uuid::Uuid::new_v4().to_string();
                let now: String = (Utc::now()).to_string();
                let new_task = Task {
                    id: task_id.clone(),
                    idcolumn: task_info.idcolumn.clone(),
                    iduser: user_id.to_owned().clone(),
                    idproject: project_id.clone(),
                    title: task_info.title.clone(),
                    description: task_info.description.clone(),
                    state: task_info.state,
                    completed_at: None,
                    created_at: now.clone(),
                    updated_at: now.clone()
                };
                let created_task = diesel::insert_into(task::table)
                    .values(&new_task)
                    .get_result::<Task>(connection);
                
            
                match created_task {
                    Ok(task) => {
                        let achievement_updated = check_update_user_achievement(user_id, "8");
                        match achievement_updated {
                            Ok(_) => Ok(task),
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(GenericError { error: false, message: err.to_string() })
                }
            } else {
                Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn update_task(task_info: &TaskInputCreate, task_id: &String, id_app: &str, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
                let task_found: Result<Task, Error> = task::table.filter(task::id.eq(task_id)).first::<Task>(connection);
                match task_found {
                    Ok(mut task) => {
                        let now: String = (Utc::now()).to_string();
                        task.title = task_info.title.clone();
                        task.description = task_info.description.clone();
                        task.state = task_info.state;
                        if task.state == 1 {
                            task.completed_at = Some(Utc::now().naive_utc());
                        } else {
                            task.completed_at = None;
                        }
                        task.updated_at = now.clone();
            
                        let updated_project = task.save_changes::<Task>(connection);
                        match updated_project {
                            Ok(_user) => Ok(GenericError {error: false, message: "Updated task successfully".to_string()}), 
                            Err(err) => Err(GenericError {error: true, message: err.to_string()})
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

pub fn delete_task(task_id: &String, id_app: &str, user_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    match app_utils::check_app_by_type(id_app, "task_app", connection) {
        Ok(project_id) => {
            if user_utils::is_admin(&project_id, user_id, connection) || user_utils::is_editor(&project_id, user_id, connection) {
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
            } else {
                Err(GenericError { error: true, message: "You have to be a member of this project and have the admin or editor role".to_string() })
            }
        },
        Err(err) => Err(err)
    }
}

pub fn total_project_tasks(project_id: &String, user_id: &str) -> Result<TotalProjectTasks, GenericError> {
    let connection = &mut establish_connection();
    if user_utils::is_member(project_id, user_id, connection) {
        let number_tasks_found = task::table
            .select(count(task::id))
            .filter(task::idproject.eq(project_id))
            .get_result::<i64>(connection);
        match number_tasks_found {
            Ok(tasks) => Ok(TotalProjectTasks { tasks: tasks as i16}),
            Err(_) => Err(GenericError { error: true, message: "Something went wrong".to_string() })
        }
    } else {
        Err(GenericError { error: true, message: "You are not a member of the project".to_string() })
    }
}

pub fn month_completed_task(month: u32, project_id: &str, user_id: &str) -> Result<TotalProjectTasks, GenericError> {
    // Chrisitan cambiar esto la validación clean tuya
    if month < 1 || month > 12 {
        return Err(GenericError { error: true, message: "The month must be between 1 and 12".to_string() })
    }
    let connection = &mut establish_connection();
    if user_utils::is_member(project_id, user_id, connection) {
        let current_year = Utc::now().year();
        let mut date = NaiveDate::from_ymd_opt(current_year, month, 1);
        let mut time = NaiveTime::from_hms_opt(0, 0, 0);
        // Desde qué fecha
        let from_datetime = NaiveDateTime::new(date.clone().unwrap(), time.clone().unwrap());

        let last_day_of_month = NaiveDate::from_ymd_opt(Utc::now().year(), month + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(Utc::now().year() + 1, 1, 1).unwrap())
            .pred_opt();
        date = NaiveDate::from_ymd_opt(current_year, month, last_day_of_month.unwrap().day());
        println!("{}", date.unwrap().to_string());
        time = NaiveTime::from_hms_opt(23, 59, 59);
        // Hasta qué fecha
        let to_datetime = NaiveDateTime::new(date.clone().unwrap(), time.clone().unwrap());
        
        let number_task_month = task::table
        .select(count(task::id))
        .filter(task::idproject.eq(project_id))
        .filter(task::completed_at.ge(from_datetime))
        .filter(task::completed_at.le(to_datetime))
        .filter(task::state.eq(1))
        .get_result::<i64>(connection);
    match number_task_month {
        Ok(tasks) => Ok(TotalProjectTasks { tasks: tasks as i16}),
        Err(_) => Err(GenericError { error: true, message: "Something went wrong".to_string() })
    }
    } else {
        Err(GenericError { error: true, message: "You are not a member of the project".to_string() })
    }
}

pub fn total_user_project_tasks(project_id: &String, user_id: &str, token_user_id: &str) -> Result<TotalProjectTasks, GenericError> {
    let connection = &mut establish_connection();
    if user_utils::is_member(project_id, token_user_id, connection) {
        let number_tasks_found = task::table
            .select(count(task::id))
            .filter(task::idproject.eq(project_id))
            .filter(task::iduser.eq(user_id))
            .get_result::<i64>(connection);
        match number_tasks_found {
            Ok(tasks) => Ok(TotalProjectTasks { tasks: tasks as i16}),
            Err(_) => Err(GenericError { error: true, message: "Something went wrong".to_string() })
        }
    } else {
        Err(GenericError { error: true, message: "You are not a member of the project".to_string() })
    }
}