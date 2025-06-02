use crate::asg::{ASG, Node, Edge};
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{Read, BufReader};

/// Простая функция для чтения VarInt (LEB128)
fn read_varint<R: Read>(reader: &mut R) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 { break; }
        shift += 7;
    }
    Ok(result)
}

/// Загрузить ASG из бинарного файла SYN1
pub fn load_synapse_file(path: &str) -> Result<ASG> {
    let mut file = BufReader::new(File::open(path)?);

    // 1. Проверяем заголовок
    let mut header = [0u8; 4];
    file.read_exact(&mut header)?;
    if &header != b"SYN1" {
        return Err(anyhow!("Неверный заголовок SYN1"));
    }

    // 2. Версия (varint)
    let _version = read_varint(&mut file)?;

    // 3. Количество узлов (varint)
    let num_nodes = read_varint(&mut file)?;

    // 4. Считываем узлы
    let mut nodes = Vec::with_capacity(num_nodes as usize);
    for _ in 0..num_nodes {
        let node_type_code = read_varint(&mut file)?;
        let node_id = read_varint(&mut file)?;
        let payload_len = read_varint(&mut file)? as usize;
        let mut payload = vec![0u8; payload_len];
        if payload_len > 0 {
            file.read_exact(&mut payload)?;
        }
        let edge_count = read_varint(&mut file)? as usize;
        let mut edges = Vec::with_capacity(edge_count);
        for _ in 0..edge_count {
            let edge_type_code = read_varint(&mut file)?;
            let target_node_id = read_varint(&mut file)?;
            let edge_payload_len = read_varint(&mut file)? as usize;
            let edge_payload = if edge_payload_len > 0 {
                let mut buf = vec![0u8; edge_payload_len];
                file.read_exact(&mut buf)?;
                Some(buf)
            } else {
                None
            };
            edges.push(Edge {
                edge_type_code,
                target_node_id,
                payload: edge_payload,
            });
        }
        nodes.push(Node {
            id: node_id,
            node_type_code,
            payload,
            edges,
        });
    }

    // 5. Определяем entry (по умолчанию — последний узел, если нет отдельной логики)
    let entry = nodes.last().map(|n| n.id).unwrap_or(0);

    Ok(ASG { nodes, entry })
}
