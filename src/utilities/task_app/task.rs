use crate::schema::*;
use diesel::prelude::*;
use indexmap::IndexMap;
use bigdecimal::FromPrimitive;
use chrono::{Datelike, Month};
use crate::models::models::Task;

pub fn get_last_task_order(column_id: &String, connection: &mut PgConnection) -> i32{
    let last_order = task::table
        .select(task::ordering)
        .filter(task::idcolumn.eq(column_id))
        .order(task::ordering.desc())
        .first::<i32>(connection);

    match last_order {
        Ok(order) => order + 1,
        Err(_) => 1
    }
}

fn get_months_map() -> IndexMap<String, i64> {
    IndexMap::from([
        ("January".to_owned(), 0),
        ("February".to_owned(), 0),
        ("March".to_owned(), 0),
        ("April".to_owned(), 0),
        ("May".to_owned(), 0),
        ("June".to_owned(), 0),
        ("July".to_owned(), 0),
        ("August".to_owned(), 0),
        ("September".to_owned(), 0),
        ("October".to_owned(), 0),
        ("November".to_owned(), 0),
        ("December".to_owned(), 0)
    ])
}

pub fn get_tasks_per_month(tasks: Vec<Task>) -> IndexMap<String, i64> {
    let mut months_tasks: IndexMap<String, i64> = get_months_map();

    for task in tasks {
        let month = Month::from_u32(task.completed_at.unwrap().month()).unwrap();
        let month_name = month.name().to_owned();
        
        let saved_month = months_tasks.entry(month_name).or_insert(1);
        *saved_month += 1;
    }
    months_tasks
}