use crate::asg::{ASG, Node, Edge};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

/// Сериализация VarInt (маленькое число -> 1 байт, большое -> больше)
fn write_varint(mut value: u64, writer: &mut File) -> Result<()> {
    while value >= 0x80 {
        writer.write_all(&[((value as u8) & 0x7F) | 0x80])?;
        value >>= 7;
    }
    writer.write_all(&[value as u8])?;
    Ok(())
}

/// Основная функция: записать ASG в файл SYN1
pub fn save_synapse_file(path: &str, asg: &ASG) -> Result<()> {
    let mut file = File::create(path)?;

    // 1. Header
    file.write_all(b"SYN1")?;

    // 2. Version
    write_varint(1, &mut file)?;

    // 3. Количество узлов
    write_varint(asg.nodes.len() as u64, &mut file)?;

    // 4. Каждый узел
    for node in &asg.nodes {
        write_varint(node.node_type_code, &mut file)?;
        write_varint(node.id, &mut file)?;
        write_varint(node.payload.len() as u64, &mut file)?;
        file.write_all(&node.payload)?;

        write_varint(node.edges.len() as u64, &mut file)?;
        for edge in &node.edges {
            write_varint(edge.edge_type_code, &mut file)?;
            write_varint(edge.target_node_id, &mut file)?;
            let payload_len = edge.payload.as_ref().map(|v| v.len()).unwrap_or(0);
            write_varint(payload_len as u64, &mut file)?;
            if let Some(data) = &edge.payload {
                file.write_all(data)?;
            }
        }
    }

    Ok(())
}
