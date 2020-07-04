table! {
    challenge_tags (id) {
        id -> Binary,
        challenge -> Binary,
        tag -> Binary,
        created -> Datetime,
    }
}

table! {
    challenges (id) {
        id -> Binary,
        name -> Varchar,
        description -> Text,
        created -> Datetime,
    }
}

table! {
    sessions (id) {
        id -> Binary,
        token -> Binary,
        user -> Binary,
        expires -> Datetime,
        created -> Datetime,
    }
}

table! {
    solves (id) {
        id -> Binary,
        challenge -> Binary,
        user -> Binary,
        created -> Datetime,
    }
}

table! {
    tags (id) {
        id -> Binary,
        name -> Varchar,
        description -> Text,
        created -> Datetime,
    }
}

table! {
    users (id) {
        id -> Binary,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        email_validated -> Bool,
        created -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    challenge_tags,
    challenges,
    sessions,
    solves,
    tags,
    users,
);
