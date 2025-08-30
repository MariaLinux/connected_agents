use crate::config::{Node, WorkflowData};
use crate::registry::NodeRegistry;
use serde_json::json;

pub struct NodeExecutor {
    registry: NodeRegistry,
}

impl NodeExecutor {
 pub async fn execute_node(node: &Node, input_data: WorkflowData) -> Result<WorkflowData, Box<dyn std::error::Error>> {
 println!("🔄 Executing node {} ({})", node.id, node.name);
 
 match node.node_type.as_str() {
 "trigger" => Self::execute_trigger(node, input_data).await,
 "http" => Self::execute_http(node, input_data).await,
 "function" => Self::execute_function(node, input_data).await,
 _ => {
 println!("⚠️ Unknown node type: {}", node.node_type);
 Ok(input_data)
 }
 }
 }

 async fn execute_trigger(_node: &Node, _input_data: WorkflowData) -> Result<WorkflowData, Box<dyn std::error::Error>> {
 println!("✅ Trigger: Workflow started");
 Ok(WorkflowData::new())
 }

 async fn execute_http(node: &Node, input_data: WorkflowData) -> Result<WorkflowData, Box<dyn std::error::Error>> {
 let url = node.parameters
 .get("url")
 .and_then(|v| v.as_str())
 .unwrap_or("https://jsonplaceholder.typicode.com/posts/1");
 
 let method = node.parameters
 .get("method")
 .and_then(|v| v.as_str())
 .unwrap_or("GET");

 println!("🌐 HTTP {}: {}", method, url);

 match method.to_uppercase().as_str() {
 "GET" => {
 println!("📡 Simulating HTTP GET request...");
 
 let mock_response = json!({
 "id": 1,
 "title": "Sample Data",
 "body": "This is sample data from the API",
 "userId": 1
 });

 let mut output_data = input_data;
 output_data.items = vec![mock_response];
 
 println!("✅ HTTP request completed");
 Ok(output_data)
 }
 _ => {
 println!("⚠️ HTTP method {} not implemented", method);
 Ok(input_data)
 }
 }
 }

 async fn execute_function(node: &Node, input_data: WorkflowData) -> Result<WorkflowData, Box<dyn std::error::Error>> {
 let _code = node.parameters
 .get("code")
 .and_then(|v| v.as_str())
 .unwrap_or("");

 println!("⚙️ Function: Processing data...");
 
 let mut processed_items = Vec::new();
 
 for item in input_data.items {
 let mut processed_item = item;
 if let Some(json_obj) = processed_item.as_object_mut() {
 json_obj.insert("processed".to_string(), json!(true));
 // Use a simple timestamp instead of chrono for now
 json_obj.insert("processed_at".to_string(), json!("2024-01-01T00:00:00Z"));
 }
 processed_items.push(processed_item);
 }

 let output_data = WorkflowData {
 items: processed_items,
 };

 println!("✅ Function execution completed");
 Ok(output_data)
 }
}