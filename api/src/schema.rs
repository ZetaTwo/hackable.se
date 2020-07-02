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
        token -> Nullable<Binary>,
        user -> Nullable<Binary>,
        expires -> Datetime,
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
        password -> Varchar,
        email -> Varchar,
        email_validated -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    challenge_tags,
    challenges,
    sessions,
    tags,
    users,
);
