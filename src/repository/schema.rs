table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
