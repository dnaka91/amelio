table! {
    comments (id) {
        id -> Integer,
        ticket_id -> Integer,
        creator_id -> Integer,
        timestamp -> Text,
        message -> Text,
    }
}

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
    medium_interactives (ticket_id) {
        ticket_id -> Integer,
        url -> Text,
    }
}

table! {
    medium_questionaires (ticket_id) {
        ticket_id -> Integer,
        question -> Integer,
        answer -> Text,
    }
}

table! {
    medium_recordings (ticket_id) {
        ticket_id -> Integer,
        time -> Text,
    }
}

table! {
    medium_texts (ticket_id) {
        ticket_id -> Integer,
        page -> Integer,
        line -> Integer,
    }
}

table! {
    samples (id) {
        id -> Text,
        created -> Bool,
    }
}

table! {
    tickets (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        title -> Text,
        description -> Text,
        category -> Text,
        priority -> Text,
        status -> Text,
        course_id -> Integer,
        creator_id -> Integer,
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

joinable!(comments -> tickets (ticket_id));
joinable!(comments -> users (creator_id));
joinable!(medium_interactives -> tickets (ticket_id));
joinable!(medium_questionaires -> tickets (ticket_id));
joinable!(medium_recordings -> tickets (ticket_id));
joinable!(medium_texts -> tickets (ticket_id));
joinable!(tickets -> courses (course_id));
joinable!(tickets -> users (creator_id));

allow_tables_to_appear_in_same_query!(
    comments,
    courses,
    medium_interactives,
    medium_questionaires,
    medium_recordings,
    medium_texts,
    samples,
    tickets,
    users,
);
