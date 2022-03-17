table! {
    books (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        price -> Int4,
        rating -> Int4,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        downloads_count -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    packages,
);
