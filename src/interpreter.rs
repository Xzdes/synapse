use crate::asg::{ASG};
use crate::nodecodes::{NodeType, EdgeType};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Выполнение ASG: поддержка LiteralInt, LiteralFloat, BinaryAdd, BinarySub, BinaryEq, BinaryGt, Print, Conditional
pub fn execute(asg: &ASG) -> Result<()> {
    let mut int_values: HashMap<u64, i64> = HashMap::new();
    let mut float_values: HashMap<u64, f64> = HashMap::new();

    for node in &asg.nodes {
        match NodeType::from_code(node.node_type_code) {
            Some(NodeType::LiteralInt) => {
                let value = i64::from_le_bytes(slice_to_8(&node.payload));
                int_values.insert(node.id, value);
                println!("Node {}: LiteralInt = {}", node.id, value);
            }
            Some(NodeType::LiteralFloat) => {
                let value = f64::from_le_bytes(slice_to_8(&node.payload));
                float_values.insert(node.id, value);
                println!("Node {}: LiteralFloat = {}", node.id, value);
            }
            Some(NodeType::BinaryAdd) => {
                let left_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("BinaryAdd: missing left operand"))?.target_node_id;
                let right_id = node.edges.get(1)
                    .ok_or_else(|| anyhow!("BinaryAdd: missing right operand"))?.target_node_id;
                if let (Some(&left), Some(&right)) = (int_values.get(&left_id), int_values.get(&right_id)) {
                    let sum = left + right;
                    int_values.insert(node.id, sum);
                    println!("Node {}: BinaryAdd (int) = {} + {} = {}", node.id, left, right, sum);
                } else if let (Some(&left), Some(&right)) = (float_values.get(&left_id), float_values.get(&right_id)) {
                    let sum = left + right;
                    float_values.insert(node.id, sum);
                    println!("Node {}: BinaryAdd (float) = {} + {} = {}", node.id, left, right, sum);
                } else {
                    println!("Node {}: BinaryAdd: operands not found", node.id);
                }
            }
            Some(NodeType::BinarySub) => {
                let left_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("BinarySub: missing left operand"))?.target_node_id;
                let right_id = node.edges.get(1)
                    .ok_or_else(|| anyhow!("BinarySub: missing right operand"))?.target_node_id;
                if let (Some(&left), Some(&right)) = (int_values.get(&left_id), int_values.get(&right_id)) {
                    let diff = left - right;
                    int_values.insert(node.id, diff);
                    println!("Node {}: BinarySub (int) = {} - {} = {}", node.id, left, right, diff);
                } else if let (Some(&left), Some(&right)) = (float_values.get(&left_id), float_values.get(&right_id)) {
                    let diff = left - right;
                    float_values.insert(node.id, diff);
                    println!("Node {}: BinarySub (float) = {} - {} = {}", node.id, left, right, diff);
                } else {
                    println!("Node {}: BinarySub: operands not found", node.id);
                }
            }
            Some(NodeType::BinaryEq) => {
                let left_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("BinaryEq: missing left operand"))?.target_node_id;
                let right_id = node.edges.get(1)
                    .ok_or_else(|| anyhow!("BinaryEq: missing right operand"))?.target_node_id;
                if let (Some(&left), Some(&right)) = (int_values.get(&left_id), int_values.get(&right_id)) {
                    let res = if left == right { 1 } else { 0 };
                    int_values.insert(node.id, res);
                    println!("Node {}: BinaryEq (int) = {} == {} => {}", node.id, left, right, res);
                } else if let (Some(&left), Some(&right)) = (float_values.get(&left_id), float_values.get(&right_id)) {
                    let res = if (left - right).abs() < 1e-12 { 1 } else { 0 }; // для float используем эпсилон
                    int_values.insert(node.id, res);
                    println!("Node {}: BinaryEq (float) = {} == {} => {}", node.id, left, right, res);
                } else {
                    println!("Node {}: BinaryEq: operands not found", node.id);
                }
            }
            Some(NodeType::BinaryGt) => {
                let left_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("BinaryGt: missing left operand"))?.target_node_id;
                let right_id = node.edges.get(1)
                    .ok_or_else(|| anyhow!("BinaryGt: missing right operand"))?.target_node_id;
                if let (Some(&left), Some(&right)) = (int_values.get(&left_id), int_values.get(&right_id)) {
                    let res = if left > right { 1 } else { 0 };
                    int_values.insert(node.id, res);
                    println!("Node {}: BinaryGt (int) = {} > {} => {}", node.id, left, right, res);
                } else if let (Some(&left), Some(&right)) = (float_values.get(&left_id), float_values.get(&right_id)) {
                    let res = if left > right { 1 } else { 0 };
                    int_values.insert(node.id, res);
                    println!("Node {}: BinaryGt (float) = {} > {} => {}", node.id, left, right, res);
                } else {
                    println!("Node {}: BinaryGt: operands not found", node.id);
                }
            }
            Some(NodeType::PerformIOWriteLine) => {
                let input_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("PerformIOWriteLine: missing input"))?.target_node_id;
                if let Some(&value) = int_values.get(&input_id) {
                    println!("Output: {}", value);
                } else if let Some(&value) = float_values.get(&input_id) {
                    println!("Output: {}", value);
                } else {
                    println!("Output: (значение не найдено для id {})", input_id);
                }
            }
            Some(NodeType::Conditional) => {
                let cond_id = node.edges.iter()
                    .find(|e| e.edge_type_code == EdgeType::Condition.code())
                    .ok_or_else(|| anyhow!("Conditional: missing cond"))?.target_node_id;
                let then_id = node.edges.iter()
                    .find(|e| e.edge_type_code == EdgeType::ThenBranch.code())
                    .ok_or_else(|| anyhow!("Conditional: missing then"))?.target_node_id;
                let else_id = node.edges.iter()
                    .find(|e| e.edge_type_code == EdgeType::ElseBranch.code())
                    .ok_or_else(|| anyhow!("Conditional: missing else"))?.target_node_id;
                let cond = int_values.get(&cond_id)
                    .copied()
                    .unwrap_or(0);
                if cond != 0 {
                    if let Some(&res) = int_values.get(&then_id) {
                        int_values.insert(node.id, res);
                        println!("Node {}: Conditional (cond {} != 0) => then ({})", node.id, cond, res);
                    } else if let Some(&res) = float_values.get(&then_id) {
                        float_values.insert(node.id, res);
                        println!("Node {}: Conditional (cond {} != 0) => then ({})", node.id, cond, res);
                    }
                } else {
                    if let Some(&res) = int_values.get(&else_id) {
                        int_values.insert(node.id, res);
                        println!("Node {}: Conditional (cond {} == 0) => else ({})", node.id, cond, res);
                    } else if let Some(&res) = float_values.get(&else_id) {
                        float_values.insert(node.id, res);
                        println!("Node {}: Conditional (cond {} == 0) => else ({})", node.id, cond, res);
                    }
                }
            }
            None => {
                println!("Node {}: Неизвестный node_type_code {}", node.id, node.node_type_code);
            }
        }
    }

    // Вывод результата entry node, если есть
    if let Some(entry_node) = asg.nodes.iter().find(|n| n.id == asg.entry) {
        if let Some(result) = int_values.get(&entry_node.id) {
            println!("\nРезультат entry node ({}): {}", asg.entry, result);
        } else if let Some(result) = float_values.get(&entry_node.id) {
            println!("\nРезультат entry node ({}): {}", asg.entry, result);
        }
    }

    Ok(())
}

fn slice_to_8(payload: &[u8]) -> [u8; 8] {
    let mut array = [0u8; 8];
    for (i, byte) in payload.iter().enumerate().take(8) {
        array[i] = *byte;
    }
    array
}
