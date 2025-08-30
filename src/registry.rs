use crate::traits::{NodeExecutor, NodeExecutorFactory};

use std::collections::HashMap;

pub struct NodeRegistry {
    factories: HashMap<String, Box<dyn NodeExecutorFactory>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, factory: F) 
    where 
        F: NodeExecutorFactory + 'static 
    {
        let node_type = factory.supported_type().to_string();
        self.factories.insert(node_type, Box::new(factory));
    }
    // pub fn register(&mut self, factory: &dyn NodeExecutorFactory) {
    //     let node_type = factory.supported_type().to_string();
    //     self.factories.insert(node_type, factory.as_ref());
    // }

    pub fn create_executor(&self, node_type: &str) -> Option<Box<dyn NodeExecutor>> {
        self.factories.get(node_type).map(|factory| factory.create())
    }

    pub fn supported_types(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
}