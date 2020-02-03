table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}