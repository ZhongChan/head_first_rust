// @generated automatically by Diesel CLI.

diesel::table! {
    follows (follower_id, followee_id) {
        follower_id -> Int4,
        followee_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        bio -> Nullable<Text>,
        image -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    follows,
    tags,
    users,
);
