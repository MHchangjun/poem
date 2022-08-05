table! {
    poem (id) {
        id -> Text,
        subjectId -> Text,
        body -> Text,
        like -> Integer,
    }
}

table! {
    subjects (id) {
        id -> Text,
        subject -> Text,
        dt -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    poem,
    subjects,
);
