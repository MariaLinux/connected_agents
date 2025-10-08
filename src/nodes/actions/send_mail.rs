use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::{WorkflowData};

use std::pin::Pin;
use serde_yaml::Value;

pub struct ActionSendMailExecutor;
pub struct ActionSendMailFactory;

impl NodeExecutor for ActionSendMailExecutor {

    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let output_data = input;
            Ok(output_data)
        })
    }
}


impl NodeExecutorFactory for ActionSendMailFactory {
    fn create(&self) -> Box<dyn NodeExecutor> {
        Box::new(ActionSendMailExecutor{})
    }

    fn supported_type(&self) -> &'static str {
        "action"
    }

    fn plugin_name(&self) -> &'static str {
        "send_mail"
    }
}
