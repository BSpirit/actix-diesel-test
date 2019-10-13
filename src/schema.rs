table! {
    products (id) {
        id -> Integer,
        name -> Text,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    users,
);