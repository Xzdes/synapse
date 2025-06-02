use crate::nodecodes::{NodeType, EdgeType};

#[derive(Debug, Clone)]
pub struct Edge {
    pub edge_type_code: u64,
    pub target_node_id: u64,
    pub payload: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub node_type_code: u64,
    pub payload: Vec<u8>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct ASG {
    pub nodes: Vec<Node>,
    pub entry: u64,
}

impl Node {
    pub fn literal_int(id: u64, value: i64) -> Self {
        let payload = value.to_le_bytes().to_vec();
        Node {
            id,
            node_type_code: NodeType::LiteralInt.code(),
            payload,
            edges: vec![],
        }
    }

    pub fn literal_float(id: u64, value: f64) -> Self {
        let payload = value.to_le_bytes().to_vec();
        Node {
            id,
            node_type_code: NodeType::LiteralFloat.code(),
            payload,
            edges: vec![],
        }
    }

    pub fn binary_add(id: u64, left: u64, right: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::BinaryAdd.code(),
            payload: vec![],
            edges: vec![
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: left,
                    payload: None,
                },
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: right,
                    payload: None,
                },
            ],
        }
    }

    pub fn binary_sub(id: u64, left: u64, right: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::BinarySub.code(),
            payload: vec![],
            edges: vec![
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: left,
                    payload: None,
                },
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: right,
                    payload: None,
                },
            ],
        }
    }

    pub fn perform_io_write_line(id: u64, input: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::PerformIOWriteLine.code(),
            payload: vec![],
            edges: vec![
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: input,
                    payload: None,
                }
            ],
        }
    }

    pub fn conditional(id: u64, cond: u64, then_branch: u64, else_branch: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::Conditional.code(),
            payload: vec![],
            edges: vec![
                Edge { edge_type_code: EdgeType::Condition.code(), target_node_id: cond, payload: None },
                Edge { edge_type_code: EdgeType::ThenBranch.code(), target_node_id: then_branch, payload: None },
                Edge { edge_type_code: EdgeType::ElseBranch.code(), target_node_id: else_branch, payload: None },
            ],
        }
    }

    pub fn binary_eq(id: u64, left: u64, right: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::BinaryEq.code(),
            payload: vec![],
            edges: vec![
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: left,
                    payload: None,
                },
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: right,
                    payload: None,
                },
            ],
        }
    }

    pub fn binary_gt(id: u64, left: u64, right: u64) -> Self {
        Node {
            id,
            node_type_code: NodeType::BinaryGt.code(),
            payload: vec![],
            edges: vec![
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: left,
                    payload: None,
                },
                Edge {
                    edge_type_code: EdgeType::DataInput.code(),
                    target_node_id: right,
                    payload: None,
                },
            ],
        }
    }
}
