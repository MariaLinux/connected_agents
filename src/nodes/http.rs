use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::WorkflowData;

use serde_yaml::Value;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;

pub struct HttpExecutor;

impl NodeExecutor for HttpExecutor {
    // fn node_type(&self) -> &'static str {
    //     "http"
    // }

    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let url = parameters
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("https://jsonplaceholder.typicode.com/posts/1");
            
            let method = parameters
                .get("method")
                .and_then(|v| v.as_str())
                .unwrap_or("GET");

            println!("🌐 HTTP {}: {}", method, url);
            println!("📡 Simulating HTTP request...");
            
            // Simulate API response
            let mock_response = json!({
                "id": 1,
                "title": "Sample Data",
                "body": "This is sample data from the API",
                "userId": 1,
                "timestamp": "2024-01-01T00:00:00Z"
            });

            let mut output_data = input;
            output_data.items = vec![mock_response];
            
            println!("✅ HTTP request completed");
            Ok(output_data)
        })
    }
}

pub struct HttpFactory;

impl NodeExecutorFactory for HttpFactory {
    fn create(&self) -> Box<dyn NodeExecutor> {
        Box::new(HttpExecutor)
    }

    fn supported_type(&self) -> &'static str {
        "action"
    }
    
    fn plugin_name(&self) -> &'static str {
        "http"
    }
}

//TODO: make actions requests in a separate crate
//TODO: implement http requests