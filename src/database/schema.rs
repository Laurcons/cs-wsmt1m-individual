// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        #[max_length = 1024]
        title -> Varchar,
        is_done -> Bool,
    }
}
