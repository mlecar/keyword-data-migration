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
table! {
    migration_statistics (id) {
        id -> BigInt,
        unused_count -> BigInt,
        migrated_from_id -> BigInt,
        migrated_to_id -> BigInt,
    }
}