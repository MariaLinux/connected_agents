use crate::traits::{NodeExecutor, NodeExecutorFactory};

use std::collections::HashMap;

pub struct NodeRegistry {
    factories: HashMap<String, Box<dyn NodeExecutorFactory>>,
    enabled_plugins: Vec<String>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            enabled_plugins: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, factory: F) 
    where 
        F: NodeExecutorFactory + 'static 
    {
        let node_type = factory.supported_type().to_string();
        let plugin_name = factory.plugin_name().to_string();
        self.enabled_plugins.push(plugin_name);
        self.factories.insert(node_type, Box::new(factory));
    }

    pub fn create_executor(&self, node_type: &str) -> Option<Box<dyn NodeExecutor>> {
        self.factories.get(node_type).map(|factory| factory.create())
    }

    pub fn supported_types(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
    
    pub fn enabled_plugins(&self) -> Vec<&str> {
        self.enabled_plugins.iter().map(|s| s.as_str()).collect()
    }
}