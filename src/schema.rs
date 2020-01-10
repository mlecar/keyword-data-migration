table! {
    keyword (id) {
        id -> BigInt,
        #[sql_name = "keyword"]
        keyword_str -> Text,
    }
}
table! {
    unused_keyword_id (id) {
        id -> BigInt,
    }
}