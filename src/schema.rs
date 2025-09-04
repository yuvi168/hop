// @generated automatically by Diesel CLI.

diesel::table! {
    paths (rowid) {
        rowid -> Integer,
        path -> Text,
        score -> Integer,
    }
}
