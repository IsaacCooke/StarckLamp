use diesel::prelude::*;
use crate::models::lamps::{Lamp, NewLamp};
use crate::data::postgresql::establish_connection;

pub fn get_lamps(limit: i64) -> Vec<Lamp> {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();
    let results = lamps
        .limit(limit)
        .load::<Lamp>(connection)
        .expect("Error loading lamps");

    results
}

pub fn get_lamp_by_id(param_id: i32) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();
    let result = lamps
        .find(param_id)
        .get_result::<Lamp>(connection)
        .expect("Error loading lamp");

    result
}

pub fn get_lamp_by_user(param_user_id: i32) -> Vec<Lamp> {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();
    let results = lamps
        .filter(user_id.eq(param_user_id))
        .load::<Lamp>(connection)
        .expect("Error loading lamps");
    results
}

pub fn add_lamp(name: String, description: String, user_id: Option<i32>) -> Lamp {
    use crate::schema::lamps;

    let connection = &mut establish_connection();
    let new_lamp: NewLamp = NewLamp {
        name,
        description,
        red: 255,
        green: 255,
        blue: 255,
        is_on: false,
        user_id,
    };
    diesel::insert_into(lamps::table)
        .values(&new_lamp)
        .get_result(connection)
        .expect("Error saving new lamp")
}

pub fn set_color(param_id: i32, param_red: i16, param_green: i16, param_blue: i16) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();

    // Update the color
    diesel::update(lamps.find(param_id))
        .set((red.eq(param_red), green.eq(param_green), blue.eq(param_blue)))
        .get_result(connection)
        .expect("Error updating lamp")
}

pub fn toggle_on(param_id: i32, param_is_on: bool) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();

    // Update the is_on
    diesel::update(lamps.find(param_id))
        .set(is_on.eq(param_is_on))
        .get_result(connection)
        .expect("Error updating lamp")
}

pub fn update_name(param_id: i32, param_name: String) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();

    // Update the name
    diesel::update(lamps.find(param_id))
        .set(name.eq(param_name))
        .get_result(connection)
        .expect("Error updating lamp")
}

pub fn update_description(param_id: i32, param_description: String) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();

    // Update the description
    diesel::update(lamps.find(param_id))
        .set(description.eq(param_description))
        .get_result(connection)
        .expect("Error updating lamp")
}

pub fn delete_lamp(param_id: i32) -> Lamp {
    use crate::schema::lamps::dsl::*;

    let connection = &mut establish_connection();

    // Delete the lamp
    diesel::delete(lamps.find(param_id))
        .get_result(connection)
        .expect("Error deleting lamp")
}