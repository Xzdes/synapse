use crate::asg::{ASG, Node, Edge};
use std::fs::File;
use std::io::{Write, BufWriter};
use anyhow::{Result};

/// Записать VarInt (LEB128)
fn write_varint(mut value: u64, writer: &mut impl Write) -> Result<()> {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        writer.write_all(&[byte])?;
        if value == 0 { break; }
    }
    Ok(())
}

/// Сериализовать ASG в файл SYN1
pub fn save_synapse_file(path: &str, asg: &ASG) -> Result<()> {
    let mut file = BufWriter::new(File::create(path)?);

    // 1. Заголовок
    file.write_all(b"SYN1")?;

    // 2. Версия (varint)
    write_varint(1, &mut file)?;

    // 3. Количество узлов (varint)
    write_varint(asg.nodes.len() as u64, &mut file)?;

    // 4. Узлы
    for node in &asg.nodes {
        write_varint(node.node_type_code, &mut file)?;
        write_varint(node.id, &mut file)?;
        write_varint(node.payload.len() as u64, &mut file)?;
        if !node.payload.is_empty() {
            file.write_all(&node.payload)?;
        }
        write_varint(node.edges.len() as u64, &mut file)?;
        for edge in &node.edges {
            write_varint(edge.edge_type_code, &mut file)?;
            write_varint(edge.target_node_id, &mut file)?;
            if let Some(ref payload) = edge.payload {
                write_varint(payload.len() as u64, &mut file)?;
                file.write_all(payload)?;
            } else {
                write_varint(0, &mut file)?;
            }
        }
    }

    file.flush()?;
    Ok(())
}
