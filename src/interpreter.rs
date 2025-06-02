use crate::asg::{ASG};
use anyhow::Result;

/// Minimal interpreter: just print nodes
pub fn execute(asg: &ASG) -> Result<()> {
    println!("Loaded ASG with {} nodes", asg.nodes.len());
    for node in &asg.nodes {
        println!("NodeID: {}, TypeCode: {}, Payload: {:?}, Edges: {}", 
            node.id, node.node_type_code, node.payload, node.edges.len());
    }
    Ok(())
}
