// @generated automatically by Diesel CLI.

diesel::table! {
    article_tags (article_id, tag_id) {
        article_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    articles (id) {
        id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        description -> Text,
        body -> Text,
        author_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        author_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

diesel::joinable!(article_tags -> articles (article_id));
diesel::joinable!(article_tags -> tags (tag_id));
diesel::joinable!(articles -> users (author_id));
diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_tags,
    articles,
    comments,
    follows,
    tags,
    users,
);
