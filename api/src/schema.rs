table! {
    users (id) {
        id -> Binary,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        email_validated -> Bool,
    }
}
