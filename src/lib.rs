pub mod database_connection;
pub mod keyword_service_gateway;
pub mod keywords;
pub mod migration_statistics;
pub mod unused_keywords;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

pub struct KeywordManager {
    range_start: i64,
    limit: i64,
    batch_size: u64,
    next_range: Vec<i64>
}

impl KeywordManager {
    pub fn new(range_start: i64, batch_size: u64, limit: i64) -> KeywordManager {
        let range_end= range_start + batch_size as i64;
        if range_end > limit {
            KeywordManager { range_start, limit, batch_size, next_range: (range_start..=limit).collect() }
        } else{
            KeywordManager { range_start, limit, batch_size, next_range: (range_start..=range_end).collect() }
        }
    }

    pub fn next(&mut self) -> &Vec<i64> {
        let range_end = self.range_start + self.batch_size as i64;

        if range_end > self.limit {
            self.next_range = (self.range_start..=self.limit).collect();
        } else{
            self.next_range = (self.range_start..=range_end).collect();
        }
        self.range_start = range_end;
        &self.next_range
    }

    pub fn has_next(&self) -> bool {
        !(self.next_range.last().unwrap() >= &self.limit)
    }
    pub fn last(&self) -> i64 {
        self.next_range.last().cloned().unwrap()
    }
    pub fn first(&self) -> i64 {
        self.next_range.first().cloned().unwrap()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_have_next_element() {
        let keyword_manager = KeywordManager::new(1, 2, 10);
        assert_eq!(keyword_manager.has_next(), true);
    }

    #[test]
    fn should_not_have_next_element() {
        let keyword_manager = KeywordManager::new(1, 1, 2);
        assert_eq!(keyword_manager.next_range.len(), 2);
        assert_eq!(keyword_manager.has_next(), false);
    }
}