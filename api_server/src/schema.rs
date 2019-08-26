table! {
    blogs (id) {
        id -> Varchar,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    invitations (id) {
        id -> Varchar,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (email) {
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(blogs, invitations, users,);
