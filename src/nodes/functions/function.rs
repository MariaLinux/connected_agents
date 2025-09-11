use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::WorkflowData;
use serde_yaml::Value;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;

pub struct FunctionExecutor;

impl NodeExecutor for FunctionExecutor {
    // fn node_type(&self) -> &'static str {
    //     "function"
    // }

    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let _code = parameters
                .get("code")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            println!("⚙️ Function: Processing data...");
            
            let mut processed_items = Vec::new();
            
            for item in input.items {
                let mut processed_item = item;
                if let Some(json_obj) = processed_item.as_object_mut() {
                    json_obj.insert("processed".to_string(), json!(true));
                    json_obj.insert("processed_at".to_string(), json!("2024-01-01T00:00:00Z"));
                }
                processed_items.push(processed_item);
            }

            let output_data = WorkflowData {
                items: processed_items,
            };

            println!("✅ Function execution completed");
            Ok(output_data)
        })
    }
}

pub struct FunctionFactory;

impl NodeExecutorFactory for FunctionFactory {
    fn create(&self) -> Box<dyn NodeExecutor> {
        Box::new(FunctionExecutor)
    }

    fn supported_type(&self) -> &'static str {
        "function"
    }
    
    fn plugin_name(&self) -> &'static str {
        "js_engine"
    }
}

//TODO: implement js engine