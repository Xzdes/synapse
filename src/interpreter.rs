use crate::asg::{ASG};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Выполнение ASG: вычисление результата выражения (LiteralInt, Add, Print, Conditional)
pub fn execute(asg: &ASG) -> Result<()> {
    let mut values: HashMap<u64, i64> = HashMap::new();

    for node in &asg.nodes {
        match node.node_type_code {
            1 => { // LiteralInt
                let value = i64::from_le_bytes(slice_to_8(&node.payload));
                values.insert(node.id, value);
                println!("Node {}: LiteralInt = {}", node.id, value);
            }
            10 => { // BinaryOperationAdd
                let left_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("BinaryAdd: missing left operand"))?.target_node_id;
                let right_id = node.edges.get(1)
                    .ok_or_else(|| anyhow!("BinaryAdd: missing right operand"))?.target_node_id;
                let left = *values.get(&left_id)
                    .ok_or_else(|| anyhow!("BinaryAdd: left operand not found"))?;
                let right = *values.get(&right_id)
                    .ok_or_else(|| anyhow!("BinaryAdd: right operand not found"))?;
                let sum = left + right;
                values.insert(node.id, sum);
                println!("Node {}: BinaryAdd = {} + {} = {}", node.id, left, right, sum);
            }
            100 => { // PerformIOWriteLine
                let input_id = node.edges.get(0)
                    .ok_or_else(|| anyhow!("PerformIOWriteLine: missing input"))?.target_node_id;
                let value = *values.get(&input_id)
                    .ok_or_else(|| anyhow!("PerformIOWriteLine: input not found"))?;
                println!("Output: {}", value);
            }
            20 => { // Conditional
                let cond_id = node.edges.iter()
                    .find(|e| e.edge_type_code == 2)
                    .ok_or_else(|| anyhow!("Conditional: missing cond"))?.target_node_id;
                let then_id = node.edges.iter()
                    .find(|e| e.edge_type_code == 3)
                    .ok_or_else(|| anyhow!("Conditional: missing then"))?.target_node_id;
                let else_id = node.edges.iter()
                    .find(|e| e.edge_type_code == 4)
                    .ok_or_else(|| anyhow!("Conditional: missing else"))?.target_node_id;
                let cond = *values.get(&cond_id)
                    .ok_or_else(|| anyhow!("Conditional: cond not found"))?;
                if cond != 0 {
                    let res = *values.get(&then_id)
                        .ok_or_else(|| anyhow!("Conditional: then not found"))?;
                    values.insert(node.id, res);
                    println!("Node {}: Conditional (cond {} != 0) => then ({})", node.id, cond, res);
                } else {
                    let res = *values.get(&else_id)
                        .ok_or_else(|| anyhow!("Conditional: else not found"))?;
                    values.insert(node.id, res);
                    println!("Node {}: Conditional (cond {} == 0) => else ({})", node.id, cond, res);
                }
            }
            _ => {
                println!("Node {}: Неизвестный node_type_code {}", node.id, node.node_type_code);
            }
        }
    }

    // Вывод результата entry node, если есть
    if let Some(entry_node) = asg.nodes.iter().find(|n| n.id == asg.entry) {
        if let Some(result) = values.get(&entry_node.id) {
            println!("\nРезультат entry node ({}): {}", asg.entry, result);
        }
    }

    Ok(())
}

/// Преобразовать Vec<u8> к [u8; 8] (для i64::from_le_bytes)
fn slice_to_8(payload: &[u8]) -> [u8; 8] {
    let mut array = [0u8; 8];
    for (i, byte) in payload.iter().enumerate().take(8) {
        array[i] = *byte;
    }
    array
}
