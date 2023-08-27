// @generated automatically by Diesel CLI.

diesel::table! {
    lamps (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        red -> Int2,
        green -> Int2,
        blue -> Int2,
        is_on -> Bool,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(lamps -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    lamps,
    users,
);
