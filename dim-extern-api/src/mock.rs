use crate::ExternalActor;
use crate::ExternalMedia;
use crate::ExternalQuery;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub struct MockProvider;

#[async_trait::async_trait]
impl ExternalQuery for MockProvider {
    async fn search(&self, title: &str, _: Option<i32>) -> Result<Vec<ExternalMedia>> {
        Ok(vec![ExternalMedia {
            title: title.into(),
            ..Default::default()
        }])
    }

    async fn search_by_id(&self, _: &str) -> Result<ExternalMedia> {
        unimplemented!()
    }

    async fn cast(&self, _: &str) -> Result<Vec<ExternalActor>> {
        unimplemented!()
    }
}
