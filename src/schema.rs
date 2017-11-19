table! {
    settings (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Varchar,
        value -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(settings -> users (user_id));
