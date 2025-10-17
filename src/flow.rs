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

    pub fn build_graph(&mut self, workflow: &Workflow, enabled_plugins: Vec<&str>) {
        // Add nodes to graph  
        for node in &workflow.nodes {
              if !enabled_plugins.contains(&node.plugin.as_str()) {
                  continue;
              }
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
    pub fn get_execution_order(&mut self) -> Vec<NodeIndex> {
        // Filter nodes that are part of the connections (have at least one incoming or outgoing edge)
        let connected_nodes: Vec<NodeIndex> = self.graph.node_indices()
            .filter(|&idx| self.graph.edges(idx).next().is_some() || self.graph.edges_directed(idx, petgraph::Incoming).next().is_some())
            .collect();

        let filtered = self.graph.filter_map(
            |idx, node| if connected_nodes.contains(&idx) { Some(node.clone()) } else { None },
            |_, edge| Some(edge.clone())
        );
        
        self.graph = filtered;
        self.id_to_index.clear();
        self.index_to_id.clear();
        
        for idx in self.graph.node_indices() {
            let node = &self.graph[idx];
            self.id_to_index.insert(node.id, idx);
            self.index_to_id.insert(idx, node.id);
        }
        
        // Perform a topological sort on the subgraph
        toposort(&self.graph, None)
            .expect("Subgraph should be acyclic at this point")
    }
}
