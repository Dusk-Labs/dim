// use crate::AppState;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn index() -> IndexTemplate {
    IndexTemplate {}
}
