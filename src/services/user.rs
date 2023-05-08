extern crate redis;

use std::env;
use crate::models::models::*;
use crate::utilities::jwt::*;
use crate::utilities::redis::*;

use diesel::prelude::*;
use rust_api_rest::establish_connection;
use rust_api_rest::schema::users::dsl::*;

// pub fn profile(id_string: &String) -> Result<User, String> {
//     let connection = &mut establish_connection();
//     let user_found = users.filter(id.eq(&id_string))
//     .first::<User>(connection);

//     match user_found {
//         Ok(user) => {
            
//         },
//         Err(err) => Err(err.to_string())
//     }



























// }

// Siguiente funcion

