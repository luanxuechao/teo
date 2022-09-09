use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::JsonPipelineItem;
use crate::core::pipeline::stage::Stage;

#[derive(Debug)]
pub(crate) struct SetDefaultItem {
    key: String,
    value: JsonValue,
}

impl SetDefaultItem {
    pub(crate) fn new(key: impl Into<String>, value: JsonValue) -> Self {
        Self { key: key.into(), value }
    }
}

#[async_trait]
impl JsonPipelineItem for SetDefaultItem {
    async fn call(&self, stage: Stage) -> Stage {
        todo!()
    }
}