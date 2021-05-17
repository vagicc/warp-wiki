table! {
    admins (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Bpchar,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Bpchar>,
        role -> Nullable<Int2>,
        status -> Nullable<Int8>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    linksnap (id) {
        id -> Int4,
        title -> Varchar,
        url -> Text,
        added -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        cookie -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Bpchar>,
        username -> Varchar,
        realname -> Varchar,
        password -> Varchar,
        salt -> Nullable<Bpchar>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    admins,
    linksnap,
    sessions,
    users,
);
