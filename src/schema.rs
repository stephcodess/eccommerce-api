// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        last_name -> Text,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        user_password -> Varchar,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        date_created -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
