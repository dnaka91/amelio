table! {
    courses (id) {
        id -> Integer,
        code -> Text,
        title -> Text,
        author_id -> Integer,
        tutor_id -> Integer,
        active -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        name -> Text,
        role -> Text,
        active -> Bool,
        code -> Text,
    }
}

allow_tables_to_appear_in_same_query!(courses, users,);
