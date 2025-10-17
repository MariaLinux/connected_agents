use crate::traits::{NodeExecutor, NodeExecutorFactory};
use crate::config::{WorkflowData, WorkflowDataType};

use std::error::Error;
use std::pin::Pin;
use std::str::FromStr;
use serde_yaml::Value;
use lettre::{Message, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;

pub struct ActionSendMailExecutor;
pub struct ActionSendMailFactory;

impl NodeExecutor for ActionSendMailExecutor {

    fn execute<'a>(&'a self, parameters: &'a Value, input: WorkflowData) -> Pin<Box<dyn Future<Output = Result<WorkflowData, Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            // Extract and validate required parameters
            let from = parameters
                                .get("from")
                                .and_then(|v| v.as_str())
                                .ok_or("Missing required parameter: 'from'")?;
            let to = parameters
                                .get("to")
                                .and_then(|v| v.as_str())
                                .ok_or("Missing required parameter: 'to'")?;
            let smtp_host = parameters
                                .get("smtp_host")
                                .and_then(|v| v.as_str())
                                .ok_or("Missing required parameter: 'smtp_host'")?;
            // Extract optional parameters
            let reply_to = parameters
                                .get("reply_to")
                                .and_then(|v| v.as_str())
                                .unwrap_or(from);
            let param_subject = parameters
                                .get("subject")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
            let param_body = parameters
                                .get("body")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
            
            let mut subject: Option<String> = None;
            let mut body: Option<String> = None;
            // Body from input
            match input.data_type {
                WorkflowDataType::Json => {
                    for item in &input.items {
                        if let Some(_) = item.get("metadata") {
                            continue;
                        } else {
                            body = Some(serde_json::to_string_pretty(&item).unwrap());
                            break;
                        }
                    }
                },
                WorkflowDataType::PlainText | WorkflowDataType::Html | WorkflowDataType::Xml => {
                    body = input.text;                },
                _ => {}
            }
            
            for item in &input.items {
                if let Some(metadata) = item.get("metadata") {
                    if let Some(metadata_object) = metadata.as_object() {
                        if let Some(_subject) = metadata_object.get("subject") {
                            subject = String::from_str(_subject.as_str().unwrap()).unwrap().into();
                            break;
                        }
                        if body.is_none() && let Some(_body) = metadata_object.get("body") {
                            body = String::from_str(_body.as_str().unwrap()).unwrap().into();
                            break;
                        }
                    }
                }
            }
            
            if subject.is_none() {
                subject = String::from_str(param_subject).unwrap().into();
            }
            
            if body.is_none() {
                body = String::from_str(param_body).unwrap().into();
            }
            
            println!("subject: {:?}", subject);
            println!("body: {:?}", body);
            println!("smtp_host: {}", smtp_host);
            

            // Build the email message
            let message = Message::builder()
                            .from(from.parse().map_err(|e| format!("Invalid 'from' address: {}", e))?)
                            .reply_to(reply_to.parse().map_err(|e| format!("Invalid 'reply_to' address: {}", e))?)
                            .to(to.parse().map_err(|e| format!("Invalid 'to' address: {}", e))?)
                            .header(ContentType::TEXT_PLAIN)
                            .subject(subject.unwrap().as_str())
                            .body(body.unwrap().as_bytes().to_vec())
                            .map_err(|e| format!("Failed to build email message: {}", e))?;

            
            // Configure SMTP transport
            let mut smtp_transport_builder = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
                .map_err(|e| format!("Failed to configure SMTP relay: {}", e))?;
            
            // Handle SMTP port if provided
            if let Some(port) = parameters.get("smtp_port").and_then(|v| v.as_i64()) {
                smtp_transport_builder = smtp_transport_builder.port(port as u16);
            }
            
            // Handle SMTP credentials if provided
            if let (Some(user), Some(password)) = (
                parameters.get("smtp_user").and_then(|v| v.as_str()),
                parameters.get("smtp_password").and_then(|v| v.as_str()),
            ) {
                println!("user: {}", user);
                println!("password: {}", password);
                let creds = Credentials::new(user.to_string(), password.to_string());
                smtp_transport_builder = smtp_transport_builder.credentials(creds);
            }

            let smtp_transport = smtp_transport_builder.build();


            // Send the email
            match smtp_transport.send(message).await {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => {
                    eprintln!("Error sending email: {}", e);
                    return Err(Box::new(e) as Box<dyn Error>);
                }
            }

            // Prepare the output data
            let mut output_data = WorkflowData::new();
            output_data.data_type = WorkflowDataType::None;
            output_data.items = vec![serde_json::json!({"message": "Workflow completed"})];

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
