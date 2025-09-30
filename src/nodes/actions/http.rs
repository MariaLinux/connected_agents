use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::{WorkflowData, WorkflowDataType};

use reqwest::header::CONTENT_TYPE;
use serde_yaml::Value;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;


pub struct ActionHttpExecutor;

impl NodeExecutor for ActionHttpExecutor {
    // fn node_type(&self) -> &'static str {
    //     "http"
    // }

    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let url = parameters
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("https://httpbin.org/ip");
            
            let method = parameters
                .get("method")
                .and_then(|v| v.as_str())
                .unwrap_or("GET");

            println!("🌐 HTTP {}: {}", method, url);
            let mut output_data = input;
            let mut metadata = HashMap::new();
            if method == "GET" {
                let res = reqwest::get(url).await?;
                println!("Response: {:?} {}", res.version(), res.status());
                let headers = res.headers().clone();
                let content_type = headers.get(CONTENT_TYPE).unwrap();
                println!("Headers: {:#?}\n", content_type);
                metadata.insert("metadata".to_string(), json!({
                    "status": res.status().as_u16(),
                }));
                
                let content_type_byte = content_type.as_bytes();
                if content_type_byte.starts_with(b"application/json") || content_type_byte.starts_with(b"application/ld+json") {
                    let json = res.json().await?;
                    output_data.items = vec![json, json!(metadata)];
                    output_data.data_type = WorkflowDataType::Json;
                    output_data.bytes = None;
                    output_data.text = None;
                } else if content_type_byte.starts_with(b"text/plain") {
                    output_data.items = vec![json!(metadata)];
                    let text = res.text().await?;
                    output_data.text = Some(text);
                    output_data.data_type = WorkflowDataType::PlainText;
                    output_data.bytes = None;
                } else if content_type_byte.starts_with(b"text/html") || content_type_byte.starts_with(b"application/xhtml+xml") {
                    output_data.items = vec![json!(metadata)];
                    let text = res.text().await?;
                    output_data.text = Some(text);
                    output_data.data_type = WorkflowDataType::Html;
                    output_data.bytes = None;
                } else if content_type_byte.starts_with(b"application/xml") || content_type_byte.starts_with(b"text/xml") {
                    output_data.items = vec![json!(metadata)];
                    let text = res.text().await?;
                    output_data.text = Some(text);
                    output_data.data_type = WorkflowDataType::Xml;
                    output_data.bytes = None;
                } else {
                    output_data.items = vec![json!(metadata)];
                    let bytes = res.bytes().await?;
                    output_data.bytes = Some(bytes.to_vec());
                    output_data.data_type = WorkflowDataType::Unknown;
                    output_data.text = None;
                }
            }
            
            println!("✅ HTTP request completed");
            Ok(output_data)
        })
    }
}

pub struct ActionHttpFactory;

impl NodeExecutorFactory for ActionHttpFactory {
    fn create(&self) -> Box<dyn NodeExecutor> {
        Box::new(ActionHttpExecutor)
    }

    fn supported_type(&self) -> &'static str {
        "action"
    }
    
    fn plugin_name(&self) -> &'static str {
        "http"
    }
}

//TODO: implement http requests