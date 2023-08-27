use diesel::prelude::*;
use crate::models::users::{User, NewUser};
use crate::data::postgresql::establish_connection;

pub fn get_users(limit: i64) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(limit)
        .load::<User>(connection)
        .expect("Error loading users");

    results
}

pub fn get_user_by_id(param_id: i32) -> User {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = users
        .find(param_id)
        .get_result::<User>(connection)
        .expect("Error loading user");

    result
}

pub fn add_user(username: String, email: String, password: String) -> User {
    use crate::schema::users;

    let connection = &mut establish_connection();
    let new_user: NewUser = NewUser {
        username,
        email,
        password,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn update_username(param_id: i32, param_username: String) -> User {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    // Update the username
    diesel::update(users.find(param_id))
        .set(username.eq(param_username))
        .get_result(connection)
        .expect("Error updating user")
}

pub fn update_email(param_id: i32, param_email: String) -> User {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    // Update the email
    diesel::update(users.find(param_id))
        .set(email.eq(param_email))
        .get_result(connection)
        .expect("Error updating user")
}

pub fn update_password(param_id: i32, param_password: String) -> User {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    // Update the password
    diesel::update(users.find(param_id))
        .set(password.eq(param_password))
        .get_result(connection)
        .expect("Error updating user")
}

pub fn delete_user(param_id: i32) -> User {
    use crate::schema::users::dsl::*;

    // Connect to the database
    let connection = &mut establish_connection();

    // Delete the user
    diesel::delete(users.find(param_id))
        .get_result(connection)
        .expect("Error deleting user")
}