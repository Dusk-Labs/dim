use crate::tmdb::MovieResult;

pub trait APIExec<'a> {
    fn new(api_key: &'a str) -> Self;
    fn search(&mut self, title: String, year: Option<i32>) -> Option<MovieResult>;
}
