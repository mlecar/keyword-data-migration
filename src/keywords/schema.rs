table! {
    keyword (id) {
        id -> BigInt,
        #[sql_name = "keyword"]
        keyword_str -> Text,
    }
}
