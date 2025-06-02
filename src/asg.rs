use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub node_type_code: u64,
    pub payload: Vec<u8>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub edge_type_code: u64,
    pub target_node_id: u64,
    pub payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ASG {
    pub nodes: Vec<Node>,
    pub entry: u64,
}

// --- Удобные конструкторы для простых узлов ---

impl Node {
    pub fn literal_int(id: u64, value: i64) -> Self {
        let payload = value.to_le_bytes().to_vec();
        Node {
            id,
            node_type_code: 1, // 1 == LiteralInt по спецификации (утвердим этот код)
            payload,
            edges: vec![],
        }
    }

    pub fn binary_add(id: u64, input_left: u64, input_right: u64) -> Self {
        Node {
            id,
            node_type_code: 10, // 10 == BinaryOperationAdd (например)
            payload: vec![], // для add payload может быть пустым
            edges: vec![
                Edge {
                    edge_type_code: 1, // 1 == DataInput
                    target_node_id: input_left,
                    payload: None,
                },
                Edge {
                    edge_type_code: 1, // 1 == DataInput
                    target_node_id: input_right,
                    payload: None,
                },
            ],
        }
    }
     pub fn perform_io_write_line(id: u64, input: u64) -> Self {
        Node {
            id,
            node_type_code: 100, // PerformIOWriteLine
            payload: vec![],     // Нет дополнительного payload
            edges: vec![
                Edge {
                    edge_type_code: 1, // DataInput
                    target_node_id: input,
                    payload: None,
                }
            ],
        }
    }
        /// Conditional: если cond != 0, вычисляется then, иначе else
    pub fn conditional(id: u64, cond: u64, then_branch: u64, else_branch: u64) -> Self {
        Node {
            id,
            node_type_code: 20, // Conditional
            payload: vec![],
            edges: vec![
                Edge { edge_type_code: 2, target_node_id: cond, payload: None },         // Condition
                Edge { edge_type_code: 3, target_node_id: then_branch, payload: None },  // ThenBranch
                Edge { edge_type_code: 4, target_node_id: else_branch, payload: None },  // ElseBranch
            ],
        }
    }
}
