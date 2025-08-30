use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::WorkflowData;

use serde_yaml::Value;
use std::future::Future;
use std::pin::Pin;

pub struct TriggerExecutor;

impl NodeExecutor for TriggerExecutor {
    // fn node_type(&self) -> &'static str {
    //     "trigger"
    // }

    fn execute<'a>(&'a self, _parameters: &'a Value, _input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            println!("✅ Trigger: Workflow started");
            Ok(WorkflowData::new())
        })
    }
}

pub struct TriggerFactory;

impl NodeExecutorFactory for TriggerFactory {
    fn create(&self) -> Box<dyn NodeExecutor> {
        Box::new(TriggerExecutor)
    }

    fn supported_type(&self) -> &'static str {
        "trigger"
    }
    
    fn plugin_name(&self) -> &'static str {
        "default"
    }
}

//TODO: make triggers in a separate crate