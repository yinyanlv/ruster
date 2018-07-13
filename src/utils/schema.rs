table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

table! {
    themes (id) {
        id -> Int4,
        user_id -> Int4,
        category_id -> Int4,
        theme_status -> Int4,
        title -> Text,
        content -> Text,
        view_count -> Int4,
        comment_count -> Int4,
        created_at -> Timestamp,
    }
}
table! {
    categorys (id) {
        id -> Int4,
        user_id -> Int4,
        category_name -> Text,
        category_name_cn -> Text,
        created_at -> Timestamp,
    }
}
table! {
    comments (id) {
        id -> Int4,
        theme_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
    }
}
table! {
    messages (id) {
        id -> Int4,
        theme_id -> Int4,
        from_user_id -> Int4,
        to_user_id -> Int4,
        content -> Text,
        types -> Int4,
        message_status -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    saves (id) {
        id -> Int4,
        theme_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}
