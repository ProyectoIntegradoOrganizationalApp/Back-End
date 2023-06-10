use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use rust_api_rest::establish_connection;
use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::achievement;
use crate::schema::achievement_user;
use crate::utilities::user::*;

// Create all thea achievements that an user will have
pub fn create_user_achievements(user_id: &str) -> Result<(), GenericError>{
    let connection = &mut establish_connection();
    let values = vec![
        UserAchievement { idachievement: "1".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "2".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "3".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "4".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "5".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "6".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "7".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "8".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "9".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "10".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "11".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false},
        UserAchievement { idachievement: "12".to_owned(), iduser: user_id.to_owned(), progress: 0, percentage: BigDecimal::from(0), current_state: 0, completed: false}
    ];

    let inserted_achievements = diesel::insert_into(achievement_user::table)
        .values(&values)
        .execute(connection);

    match inserted_achievements {
        Ok(_) => Ok(()),
        Err(_) =>  Err(GenericError { error: true, message: "An error ocurred creating user achievements".to_owned() })
    }
}

// Get all the achievements that are registered in the database 
pub fn get_all_achievements() -> Result<Vec<Achievement>, GenericError> {
    let connection = &mut establish_connection();
    
    let results = achievement::table.load::<Achievement>(connection);

    match results {
        Ok(res) => Ok(res),
        Err(_) => Err(GenericError {
            error: true,
            message: "An error ocurred while trying to get the achievements".to_owned()
        })
    }
}

// Get all the achievements related to an user
pub fn get_user_achievements_util(user: &User, connection: &mut PgConnection) -> Result<Vec<UserAchievementsInfo>, GenericError>  {
    let achievements_found = UserAchievement::belonging_to(&user)
        .inner_join(achievement::table)
        .select((Achievement::as_select(), UserAchievement::as_select()))
        .load::<(Achievement, UserAchievement)>(connection);
    
    match achievements_found {
        Ok(achievements) => {
            let mut achievements_info:Vec<UserAchievementsInfo> = Vec::new();
            for i in &achievements {
                let user_achievements_info = UserAchievementsInfo {
                    id: i.0.id.clone(),
                    title: i.0.title.clone(),
                    description: i.0.description.clone(),
                    icon: i.0.icon.clone(),
                    category: i.0.category.clone(),
                    progress: i.1.progress,
                    completed: i.1.completed,
                    current_state: i.1.current_state,
                    percentage: i.1.percentage.clone()
                };
                achievements_info.push(user_achievements_info);
            }
            Ok(achievements_info)
        },
        Err(_) => Err(GenericError { error: true, message: "An error ocurred while trying to get the achievements".to_owned() })
    }
}

pub fn check_update_user_achievement(user_id: &str, achievement_id: &str) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();

    let achievement_found = achievement::table.inner_join(achievement_user::table)
                .filter(achievement::id.eq(achievement_id.to_owned()))
                .filter(achievement_user::iduser.eq(user_id.to_owned()))
                .select((Achievement::as_select(), UserAchievement::as_select()))
                .first::<(Achievement, UserAchievement)>(connection);
    
    match achievement_found {
        Ok(result) => {
            let mut increment_experience = false;
            let (achiv, mut user_achiv) = result;
            user_achiv.progress += 1;

            let last_state = achiv.states[achiv.states.len() -1].unwrap();

            // Actualizamos el porcentaje
            if user_achiv.percentage < BigDecimal::from(100) {
                let testing = user_achiv.progress as f64 * 100.0 / last_state as f64;
                user_achiv.percentage = BigDecimal::from_f64(testing).unwrap();
            }
             
            // Si el logro no está completo
            if !user_achiv.completed {
                let current_state_value = achiv.states[user_achiv.current_state as usize].unwrap();
                // Si el progreso en el state actual es mayor que el valor max del state actual
                if user_achiv.progress >= current_state_value {
                    increment_experience = true;
                    // Si el state es el último
                    if user_achiv.current_state == achiv.states.len() as i32 - 1 {
                        // Se completa el achievement
                        user_achiv.completed = true;
                    } else {
                        // Si no es el último state, el logro avanza de state
                        user_achiv.current_state += 1;
                    }
                }
            }
            let user_achivement_updated = user_achiv.save_changes::<UserAchievement>(connection);
            match user_achivement_updated {
                Ok(_) => {
                    if increment_experience {
                        let increment_updated = increment_exp(&user_id.to_owned(), user_achiv.current_state, connection);
                        match increment_updated {
                            Ok(_) => Ok(GenericError { error: false, message: "The achievement was updated successfully".to_owned() }),
                            Err(err) => Err(err)
                        }
                    } else {
                        Ok(GenericError { error: false, message: "The achievement was updated successfully".to_owned() })
                    }
                },
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}

pub fn get_user_achievements_profile(user: &User, connection: &mut PgConnection) -> Result<Vec<UserAchievementsInfo>, String> {
    let achievements_found = UserAchievement::belonging_to(&user)
        .inner_join(achievement::table)
        .select((Achievement::as_select(), UserAchievement::as_select()))
        .filter(achievement_user::progress.gt(0))
        .load::<(Achievement, UserAchievement)>(connection);
    
    match achievements_found {
        Ok(achievements) => {
            // Pick up only the achievements' information we need
            let mut achievements_info:Vec<UserAchievementsInfo> = Vec::new();
            for i in &achievements {
                let user_achievements_info = UserAchievementsInfo {
                    id: i.0.id.clone(),
                    title: i.0.title.clone(),
                    description: i.0.description.clone(),
                    icon: i.0.icon.clone(),
                    category: i.0.category.clone(),
                    progress: i.1.progress,
                    completed: i.1.completed,
                    current_state: i.1.current_state,
                    percentage: i.1.percentage.clone()
                };
                achievements_info.push(user_achievements_info);
            }
            Ok(achievements_info)
        },
        Err(err) => Err(err.to_string())
    }
}