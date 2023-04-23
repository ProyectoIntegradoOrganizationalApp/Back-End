use std::env;
use crate::models::models::*;
use crate::utilities::jwt::*;
extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rust_api_rest::establish_connection;
use rust_api_rest::schema::users::dsl::*;
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport};

pub fn register(user_info: &UserInput) -> User {
    use crate::schema::users;
    println!("{:#?}", user_info);
    let connection = &mut establish_connection();
    let user_id = uuid::Uuid::new_v4().to_string();
    let hashed_password = hash(&user_info.password, DEFAULT_COST).unwrap();
    let new_user = User {
        id: user_id,
        name: String::from(&user_info.first_name),
        email: String::from(&user_info.email),
        password: String::from(&hashed_password)
    };

    let created_user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error while creating new user");

    created_user
}

pub fn login(user_info: &UserLogin) -> Result<String, String> {
    use crate::schema::users::dsl::*;
    println!("{:#?}", user_info );
    
    let connection = &mut establish_connection();
    let user_found = users.filter(email.eq(String::from(&user_info.email)))
    .first::<User>(connection);

    match user_found{
        Ok(user) => create_token(&user.id),
        Err(err) => Err(err.to_string())
    }
}

pub fn send_mail(user_mail: &UserMail) -> ResponseMessage {
    let connection = &mut establish_connection();
    let user = users.filter(email.like(&user_mail.email))
        .load::<User>(connection)
        .expect("An error has ocurred.");
    
    let mut message = "";

    if user.len() == 0 {
        message = "This email doesn't exist.";
    } else {
        let mail_to_send = Message::builder() 
            .from("Sender <teamerbusiness0@gmail.com>".parse().unwrap()) 
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
            },
            Err(e) => println!("{:#?}", e)
        }
    }

    let message_json = ResponseMessage {
        message: String::from(message)
    };

    message_json
}

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
        Err(e) => Err(String::from("Something went wrong"))
    }
}