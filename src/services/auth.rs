extern crate bcrypt;
extern crate redis;

use crate::models::models::*;
use crate::schema::users;
use crate::utilities::jwt::*;
use crate::utilities::redis::*;
use crate::utilities::achievements::*;
use std::env;

use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};

use diesel::prelude::*;
use rust_api_rest::establish_connection;
use rust_api_rest::schema::users::dsl::*;

use chrono::Utc;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn register(user_info: &UserInput) -> Result<User, String> {
    let now: String = (Utc::now()).to_string();
    let connection = &mut establish_connection();
    let user_id = uuid::Uuid::new_v4().to_string();
    let hashed_password = hash(&user_info.password, DEFAULT_COST).unwrap();
    let new_user = User {
        id: user_id,
        email: String::from(&user_info.email),
        password: String::from(&hashed_password),
        name: String::from(&user_info.first_name),
        lastname: String::from(&user_info.last_name),
        phone: String::from(&user_info.phone),
        created_at: now.clone(),
        updated_at: now.clone(),
        level: 1,
        photo: String::from(&user_info.photo)
    };

    let created_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection);

    match created_user {
        Ok(user) => {
            create_user_achievements(&user.id);
            Ok(user)
        },
        Err(e) => Err(e.to_string()),
    }
}

#[allow(unused)]
pub fn login(user_info: &UserLogin) -> Result<UserLoginResponse, String> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let user_found = users
        .filter(email.eq(String::from(&user_info.email)))
        .first::<User>(connection);

    match user_found {
        Ok(user) => {
            let verified_password = verify(&user_info.password, &user.password);
            match verified_password {
                Ok(correct_password) => {
                    if (correct_password) {
                        match create_token(&user.id) {
                            Ok(token) => match whitelist_token(token.as_str(), &user.id) {
                                Ok(r) => {
                                    let notifications_found = Notification::belonging_to(&user)
                                    .select(Notification::as_select())
                                    .load::<Notification>(connection);
                                    
                                    match notifications_found {
                                        Ok(notifications) => {
                                            let mut notifications_info:Vec<UserNotificationProfile> = Vec::new();
                                            for i in &notifications {
                                                let user_notifications_info = UserNotificationProfile {
                                                    id: i.id.clone(),
                                                    title: i.title.clone(),
                                                    content: i.content.clone(),
                                                    state: i.state
                                                };
                                                notifications_info.push(user_notifications_info);
                                            }

                                            let response = UserLoginResponse {
                                                id: user.id,
                                                email: user.email,
                                                name: user.name,
                                                lastname: user.lastname,
                                                phone: user.phone,
                                                created_at: user.created_at,
                                                updated_at: user.updated_at,
                                                level: user.level,
                                                _token: token,
                                                notifications: notifications_info
                                            };
                                            Ok(response)
                                        },
                                        Err(err) => Err(err.to_string())
                                    }
                                },
                                Err(err) => Err(err.to_string()),
                            },
                            Err(err) => Err(err.to_string()),
                        }
                    } else {
                        Err("Invalid credentials".to_string())
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn send_mail(user_mail: &UserMail) -> ResponseMessage {
    let connection = &mut establish_connection();
    let user = users
        .filter(email.like(&user_mail.email))
        .load::<User>(connection)
        .expect("An error has ocurred.");

    let mut message = "";

    if user.len() == 0 {
        message = "This email doesn't exist.";
    } else {
        let mail_to_send = Message::builder()
            .from("Sender <teamerspprt@gmail.com>".parse().unwrap())
            .to("Hei <sergioparejo671@gmail.com>".parse().unwrap())
            .subject("Sending email with Rust")
            .body(String::from("This is my first email"))
            .unwrap();

        let creds = Credentials::new("apikey".to_owned(), env::var("SENDGRID_API_KEY").unwrap());
        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.sendgrid.net")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&mail_to_send) {
            Ok(result) => {
                print!("dsasaddas: {:#?}", result);
            }
            Err(e) => println!("{:#?}", e),
        }
    }

    let message_json = ResponseMessage {
        message: String::from(message),
    };

    message_json
}

#[allow(unused)]
pub fn change_password(user_info: &ChangePass) -> Result<String, String> {
    let connection = &mut establish_connection();
    let mut updated: Result<usize, diesel::result::Error> = Ok(0);
    if user_info.pass == user_info.confirm_pass {
        let hashed_password = hash(&user_info.pass, DEFAULT_COST).unwrap();
        updated = diesel::update(users)
            .filter(email.eq(&user_info.mail))
            .set(password.eq(hashed_password))
            .execute(connection);
    }

    match updated {
        Ok(_) => Ok(String::from("Password changed successfully")),
        Err(e) => Err(String::from("Something went wrong")),
    }
}

pub fn logout(token: &String) -> Result<String, String> {
    match unwhitelist_token(token) {
        Ok(_) => Ok(String::from("Successfully logout")),
        Err(_e) => Err(String::from("Something went wrong")),
    }
}

pub fn update_user(user_info: &UserUpdate, token_iduser: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let user_to_update_found = users.filter(id.eq(token_iduser)).first::<User>(connection);
    match user_to_update_found {
        Ok(mut user_to_update) => {
            if user_to_update.email != user_info.email {
                user_to_update.email = user_info.email.clone();
            }

            match verify(&user_info.password, &user_to_update.password) {
                Ok(same_password) => {
                    if !same_password {
                        let hashed_password = hash(&user_info.password, DEFAULT_COST).unwrap();
                        user_to_update.password = hashed_password;
                    }
                },
                Err(_err) => ()
            };
            
            if user_to_update.name != user_info.name {
                user_to_update.name = user_info.name.clone();
            }
            
            if user_to_update.lastname != user_info.lastname {
                user_to_update.lastname = user_info.lastname.clone();
            }
            
            if user_to_update.phone != user_info.phone {
                user_to_update.phone = user_info.phone.clone();
            }
            
            let now: String = (Utc::now()).to_string();
            user_to_update.updated_at = now;

            let updated_user = user_to_update.save_changes::<User>(connection);
            match updated_user {
                Ok(_user) => Ok(GenericError {error: false, message: "Updated user successfully".to_string()}), 
                Err(err) => Err(GenericError {error: true, message: err.to_string()})
            }
        },
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}

pub fn delete_user(user_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let deleted = diesel::delete(users.filter(id.eq(user_id))).execute(connection);
    match deleted {
        Ok(_) => Ok(GenericError { error: false, message: "User deleted successfully".to_string() }),
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}