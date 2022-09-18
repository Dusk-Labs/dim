use super::ExternalActor;
use super::ExternalMedia;
use super::ExternalQuery;
use super::IntoQueryShow;
use super::Result;

#[derive(Debug, Clone, Copy)]
pub struct MockProvider;

#[async_trait::async_trait]
impl ExternalQuery for MockProvider {
    async fn search(&self, title: &str, year: Option<i32>) -> Result<Vec<ExternalMedia>> {
        Ok(vec![ExternalMedia {
            title: title.into(),
            ..Default::default()
        }])
    }

    async fn search_by_id(&self, external_id: &str) -> Result<ExternalMedia> {
        unimplemented!()
    }

    async fn cast(&self, external_id: &str) -> Result<Vec<ExternalActor>> {
        unimplemented!()
    }
}

impl IntoQueryShow for MockProvider {}
