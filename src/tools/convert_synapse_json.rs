use synapse::asg::{ASG, Node, Edge};
use synapse::syn1::{load_synapse_file};
use synapse::syn1_writer::{save_synapse_file};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
struct JsonEdge {
    edge_type_code: u64,
    target_node_id: u64,
    payload: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
struct JsonNode {
    id: u64,
    node_type_code: u64,
    payload: Vec<u8>,
    edges: Vec<JsonEdge>,
}

#[derive(Serialize, Deserialize)]
struct JsonASG {
    nodes: Vec<JsonNode>,
    entry: u64,
}

impl From<&ASG> for JsonASG {
    fn from(asg: &ASG) -> Self {
        JsonASG {
            nodes: asg.nodes.iter().map(JsonNode::from).collect(),
            entry: asg.entry,
        }
    }
}

impl From<&Node> for JsonNode {
    fn from(node: &Node) -> Self {
        JsonNode {
            id: node.id,
            node_type_code: node.node_type_code,
            payload: node.payload.clone(),
            edges: node.edges.iter().map(JsonEdge::from).collect(),
        }
    }
}

impl From<&Edge> for JsonEdge {
    fn from(edge: &Edge) -> Self {
        JsonEdge {
            edge_type_code: edge.edge_type_code,
            target_node_id: edge.target_node_id,
            payload: edge.payload.clone(),
        }
    }
}

impl From<JsonASG> for ASG {
    fn from(j: JsonASG) -> Self {
        ASG {
            nodes: j.nodes.into_iter().map(Node::from).collect(),
            entry: j.entry,
        }
    }
}

impl From<JsonNode> for Node {
    fn from(j: JsonNode) -> Self {
        Node {
            id: j.id,
            node_type_code: j.node_type_code,
            payload: j.payload,
            edges: j.edges.into_iter().map(Edge::from).collect(),
        }
    }
}

impl From<JsonEdge> for Edge {
    fn from(j: JsonEdge) -> Self {
        Edge {
            edge_type_code: j.edge_type_code,
            target_node_id: j.target_node_id,
            payload: j.payload,
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage:");
        println!("  convert_synapse_json --to-json input.synapse output.json");
        println!("  convert_synapse_json --from-json input.json output.synapse");
        return Ok(());
    }
    let mode = &args[1];
    let input = &args[2];
    let output = &args[3];

    match mode.as_str() {
        "--to-json" => {
            let asg = load_synapse_file(input)?;
            let json_asg = JsonASG::from(&asg);
            let file = BufWriter::new(File::create(output)?);
            serde_json::to_writer_pretty(file, &json_asg)?;
            println!("Экспортировано в {}", output);
        }
        "--from-json" => {
            let file = BufReader::new(File::open(input)?);
            let json_asg: JsonASG = serde_json::from_reader(file)?;
            let asg: ASG = json_asg.into();
            save_synapse_file(output, &asg)?;
            println!("Импортировано в {}", output);
        }
        _ => {
            println!("Неизвестный режим. Используй --to-json или --from-json");
        }
    }
    Ok(())
}
