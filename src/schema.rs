// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
