mod config;
mod traits;
mod registry;
mod nodes;
mod flow;

use config::{Workflow, Node, WorkflowData, Settings};
use registry::NodeRegistry;
use nodes::{TriggerFactory, HttpFactory, FunctionFactory};
use flow::FlowGraph;
use traits::NodeExecutorFactory;

use petgraph::graph::NodeIndex;
use petgraph::{Graph, Direction};

use std::collections::HashMap;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Modular Workflow Engine");
    
    let setting_yaml = fs::read_to_string("settings.yaml")?;
    let settings: Settings = serde_yaml::from_str(&setting_yaml)?;
    
    // Initialize plugin registry
    let mut registry = NodeRegistry::new();
    try_register(&mut registry, &settings, TriggerFactory);
    try_register(&mut registry, &settings, HttpFactory);
    try_register(&mut registry, &settings, FunctionFactory);
    
    println!("📦 Registered node types: {:?}", registry.supported_types());
    
    // Load workflow YAML
    let yaml_str = fs::read_to_string("workflow.yaml")?;
    let workflow: Workflow = serde_yaml::from_str(&yaml_str)?;

    println!("📋 Loaded workflow with {} nodes", workflow.nodes.len());

    let mut flow_graph = FlowGraph::new();
    flow_graph.build_graph(&workflow);

    // Validate DAG
    if !flow_graph.is_valid() {
        eprintln!("❌ Error: Workflow contains a cycle");
        std::process::exit(1);
    }
    println!("✅ DAG validation passed (no cycles)");

    // Get execution order
    let execution_order = flow_graph.get_execution_order();

    println!("📊 Execution order:");
    for (i, &node_idx) in execution_order.iter().enumerate() {
        let node = &flow_graph.graph[node_idx];
        println!("  {}. {} ({})", i + 1, node.name, node.node_type);
    }

    // Execute workflow
    println!("\n🎯 Executing workflow...");
    let result = execute_workflow(&flow_graph.graph, &execution_order, &flow_graph.index_to_id, &registry).await?;
    
    println!("\n🎉 Workflow completed successfully!");
    println!("📤 Final output: {}", serde_json::to_string_pretty(&result.items)?);

    Ok(())
}

async fn execute_workflow(
    graph: &Graph<Node, ()>,
    execution_order: &[NodeIndex],
    index_to_id: &HashMap<NodeIndex, u32>,
    registry: &NodeRegistry,
) -> Result<WorkflowData, Box<dyn std::error::Error>> {
    let mut node_outputs: HashMap<u32, WorkflowData> = HashMap::new();

    for &node_idx in execution_order {
        let node = &graph[node_idx];
        let node_id = *index_to_id.get(&node_idx).unwrap();

        // Get the appropriate executor
        let executor = registry
            .create_executor(&node.node_type)
            .ok_or_else(|| format!("No executor found for node type: {}", node.node_type))?;

        // Collect input data from predecessor nodes
        let input_data = collect_input_data(graph, node_idx, &node_outputs);

        // Execute the node
        println!("🔄 Executing node {} ({})", node.id, node.name);
        let output_data = executor.execute(&node.parameters, input_data).await?;
        
        // Store the output for successor nodes
        node_outputs.insert(node_id, output_data);

        println!("✅ Node {} completed\n", node.name);
    }

    // Return the output of the last node
    let last_node_idx = execution_order.last().unwrap();
    let last_node_id = *index_to_id.get(last_node_idx).unwrap();
    
    Ok(node_outputs.get(&last_node_id).unwrap().clone())
}

fn collect_input_data(
    graph: &Graph<Node, ()>,
    node_idx: NodeIndex,
    node_outputs: &HashMap<u32, WorkflowData>,
) -> WorkflowData {
    let predecessors: Vec<_> = graph
        .neighbors_directed(node_idx, Direction::Incoming)
        .collect();

    if predecessors.is_empty() {
        // This is a starting node (trigger)
        WorkflowData::clear()
    } else {
        // Merge data from all predecessor nodes
        let mut merged_items = Vec::new();
        
        for pred_idx in predecessors {
            let pred_node = &graph[pred_idx];
            if let Some(pred_output) = node_outputs.get(&pred_node.id) {
                merged_items.extend(pred_output.items.clone());
            }
        }

        WorkflowData {
            items: merged_items,
        }
    }
}


fn try_register<F>(registry: &mut NodeRegistry, settings: &Settings, factory: F)
where
    F: NodeExecutorFactory + 'static,
{
    if settings.plugin_settings
        .iter()
        .any(|s| (s.name == factory.plugin_name() && s.enabled) || factory.plugin_name() == "default")
    {
        println!("Enabled plugin: {}", factory.plugin_name());
        registry.register(factory);
    }
}

//TODO: add cli args