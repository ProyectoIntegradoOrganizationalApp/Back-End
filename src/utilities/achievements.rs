use rust_api_rest::establish_connection;
use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::achievement;
use crate::schema::achievement_user;

// Create all thea achievements that an user will have
pub fn create_user_achievements(user_id: &str) -> Result<(), GenericError>{
    let connection = &mut establish_connection();
    let values = vec![
        UserAchievement { idachievement: "1".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "2".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "3".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "4".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "5".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "6".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "7".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "8".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "9".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "10".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "11".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false},
        UserAchievement { idachievement: "12".to_owned(), iduser: user_id.to_owned(), progress: 0, current_state: 0, completed: false}
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
pub fn get_user_achievements(user_id: &str) -> Result<Vec<UserAchievement>, GenericError>  {
    let connection = &mut establish_connection();
    
    let results = achievement_user::table.filter(achievement_user::iduser.eq(user_id.to_owned()))
    .load::<UserAchievement>(connection);

    match results {
        Ok(res) => Ok(res),
        Err(_) => Err(GenericError {
            error: true,
            message: "An error ocurred while trying to get the achievements".to_owned()
        })
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
            let (achiv, mut user_achiv) = result;
            user_achiv.progress += 1;
            
            // Si el logro está completo
            if !user_achiv.completed {
                let current_state_value = achiv.states[user_achiv.current_state as usize].unwrap();
                // Si el progreso en el state actual es mayor que el valor max del state actual
                if user_achiv.progress >= current_state_value {
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
                Ok(_) => Ok(GenericError { error: false, message: "The achievement was updated successfully".to_owned() }),
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}