// @generated automatically by Diesel CLI.

diesel::table! {
    comments (comment_id) {
        comment_id -> Integer,
        post_id -> Integer,
        message_id -> BigInt,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Integer,
        message_id -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
