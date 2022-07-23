table! {
    appartment (id) {
        id -> Integer,
        price -> Nullable<Integer>,
        czynsz -> Nullable<Integer>,
        name -> Nullable<Text>,
        rooms -> Nullable<Integer>,
        scrapped_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    appartment,
    user,
);
