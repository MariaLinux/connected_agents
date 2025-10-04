use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::{WorkflowData, WorkflowDataType};

use reqwest::header::CONTENT_TYPE;
use reqwest::Response;
use serde_yaml::Value;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;


pub struct ActionHttpExecutor;

async fn process_response(res: Response, output_data: &mut WorkflowData)
    -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = HashMap::new();
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
    
    Ok(())
}

impl NodeExecutor for ActionHttpExecutor {

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
            match method.to_uppercase().as_str() {
            "GET" => {
                let res = reqwest::get(url).await?;
                process_response(res, &mut output_data).await?;
            }
            "POST" => {
                let client = reqwest::Client::new();

                if let Some(data) = parameters.get("data") {
                    if let Some(format) = data.get("format") {
                        match format.as_str().unwrap_or("raw") {
                            "json" => {
                                if let Some(json_str) = data.get("json_message") {
                                    let _json = json!(json_str.as_str().unwrap_or("{}"));
                                    let res = client.post(url).json(&_json).send().await?;
                                    process_response(res, &mut output_data).await?;
                                }
                            }
                            "form" => {
                                if let Some(form_parameters) = data.get("form_parameters") {
                                    if let Some(parameters) = form_parameters.as_sequence() {
                                        let mut params = HashMap::new();
                                        for parameter in parameters {
                                            if let Some(map) = parameter.as_mapping() {
                                                for (key, value) in map {
                                                    params.insert(key.as_str().unwrap_or("").to_string(), value.as_str().unwrap_or("").to_string());
                                                }
                                            } else {
                                                println!("Invalid form parameter: {:?}", parameter);
                                            }
                                        }
                                        if !params.is_empty() {
                                            let res = client.post(url).form(&params).send().await?;
                                            process_response(res, &mut output_data).await?;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if let Some(raw_message) = data.get("raw_message") {
                                    let _raw = raw_message.as_str().unwrap_or("").to_string();
                                    let res = client.post(url).body(_raw).send().await?;
                                    process_response(res, &mut output_data).await?;
                                }
                            }
                        }
                    }
                }

            }
            _ => {}
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

//TODO: implement http headers support
