use diesel::PgConnection;
use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;

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