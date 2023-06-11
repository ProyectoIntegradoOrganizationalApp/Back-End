use diesel::{PgConnection};
use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use chrono::Utc;
use crate::utilities::achievements::check_update_user_achievement;

pub fn get_column_tasks(columns: Vec<Columna>, connection: &mut PgConnection) -> Vec<ColumnTasks> {
    let mut column_tasks: Vec<ColumnTasks> = Vec::new();
    for column in &columns  {
        let found_tasks = task::table.filter(task::idcolumn.eq(&column.id)).load::<Task>(connection);
        match found_tasks {
            Ok(tasks) => {
                let column_task = ColumnTasks {
                    id: column.id.clone(),
                    title: column.title.clone(),
                    tasks
                };
                column_tasks.push(column_task);

            },
            Err(_) => {}
        }
    }
    column_tasks
}

pub fn create_default_columns(board_id: &String, user_id: &String, project_id: &String, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
    match new_column(&board_id, &user_id, &project_id, "To Do".to_string(), connection) {
        Ok(_) => {
            match new_column(&board_id, &user_id, &project_id, "Doing".to_string(), connection) {
                Ok(_) => {
                    match new_column(&board_id, &user_id, &project_id, "Done".to_string(), connection) {
                        Ok(_) => Ok(GenericError { error: false, message: "Default column created successfully".to_string() }),
                        Err(err) => Err(err)
                    }
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

pub fn new_column(board_id: &String, user_id: &String, project_id: &String, title: String, connection: &mut PgConnection) -> Result<GenericError, GenericError> {
    let column_id = uuid::Uuid::new_v4().to_string();
    let now: String = (Utc::now()).to_string();
    let new_column = Columna {
        id: column_id.clone(),
        idboard: board_id.clone(),
        iduser: user_id.clone(),
        idproject: project_id.clone(),
        title: title.clone(),
        created_at: now.clone(),
        updated_at: now.clone()
    };
    let created_column = diesel::insert_into(columna::table)
        .values(&new_column)
        .get_result::<Columna>(connection);

    match created_column {
        Ok(_column) => {
            let achievement_updated = check_update_user_achievement(user_id, "6");
            match achievement_updated {
                Ok(_) => Ok(GenericError { error: false, message: "Column created successfully".to_string() }),
                Err(err) => Err(err)
            }
        },
        Err(_) => Err(GenericError { error: false, message: "Error creating the column".to_string() })
    }
}