use rust_api_rest::establish_connection;
use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::achievement::dsl::*;
use crate::schema::achievement_user::dsl::*;

// Get all the achievements that are registered in the database 
pub fn get_all_achievements() -> Result<Vec<Achievement>, GenericError> {
    let connection = &mut establish_connection();
    
    let results = achievement.load::<Achievement>(connection);

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
    
    let results = achievement_user.filter(iduser.eq(user_id.to_owned()))
    .load::<UserAchievement>(connection);

    match results {
        Ok(res) => Ok(res),
        Err(_) => Err(GenericError {
            error: true,
            message: "An error ocurred while trying to get the achievements".to_owned()
        })
    }
}