table! {
    poems (id) {
        id -> Text,
        subject_id -> Text,
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
    poems,
    subjects,
);
