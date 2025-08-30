use crate::config::WorkflowData;

use serde_yaml::Value;
use std::future::Future;
use std::pin::Pin;

// Use a different approach for async trait objects
pub trait NodeExecutor: Send + Sync {
    // fn node_type(&self) -> &'static str;
    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>>;
}

pub trait NodeExecutorFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeExecutor>;
    fn supported_type(&self) -> &'static str;
    fn plugin_name(&self) -> &'static str;
}