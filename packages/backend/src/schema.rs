// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel::sql_types::Uuid;

    users (id) {
        id -> Uuid,
        password_hash -> Varchar,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
