use crate::tmdb::QueryResult;

pub trait APIExec<'a> {
    fn new(api_key: &'a str) -> Self;
    fn search(&mut self, title: String, year: Option<i32>, tv: bool) -> Option<QueryResult>;
}
