use crate::keywords::schema::keyword;

#[derive(Insertable)]
#[table_name = "keyword"]
pub struct Keyword<'a> {
    pub id: i64,
    pub keyword_str: &'a str,
}
