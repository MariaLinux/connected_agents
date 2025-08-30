use crate::config::{Workflow, Node};

use petgraph::graph::NodeIndex;
use petgraph::{Graph};
use petgraph::algo::{is_cyclic_directed, toposort};
use std::collections::HashMap;

pub struct FlowGraph {
    pub graph: Graph<Node, ()>,
    pub id_to_index: HashMap<u32, NodeIndex>,
    pub index_to_id: HashMap<NodeIndex, u32>,
}

impl FlowGraph {
    pub fn new() -> Self {
        Self { 
            graph: Graph::<Node, ()>::new(),
            id_to_index: HashMap::new(), 
            index_to_id: HashMap::new() 
        }
    }

    pub fn build_graph(&mut self, workflow: &Workflow) {
        // Add nodes to graph  
        for node in &workflow.nodes {
              let idx = self.graph.add_node(node.clone());
              self.id_to_index.insert(node.id, idx);
              self.index_to_id.insert(idx, node.id);
          }
          
          // Add edges
          for (key, targets) in &workflow.connections {
              let from_id: u32 = key.parse().unwrap();
              let from_idx = *self.id_to_index.get(&from_id).unwrap();
              
              for conn in targets {
                  if let Some(&to_idx) = self.id_to_index.get(&conn.to) {
                      self.graph.add_edge(from_idx, to_idx, ());
                  }
              }
          } 
    }

    // Validate DAG
    pub fn is_valid(&self) -> bool {
        !is_cyclic_directed(&self.graph)
    }

    // Get execution order
    pub fn get_execution_order(&self) -> Vec<NodeIndex> {
        toposort(&self.graph, None)
            .expect("Graph should be acyclic at this point")
    }
}



