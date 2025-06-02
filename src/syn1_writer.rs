// src/syn1_writer.rs

//! Сохранение ASG-графа Synapse в бинарный формат SYN1.
//! Записываются только существующие поля: id, code, value, ty для Node; from, to, code для Edge.

use std::fs::File;
use std::io::{self, Write, BufWriter};
use crate::asg::{ASG};
use crate::types::SynType;

/// Запись VarInt (compact format).
fn write_varint(mut n: u64, writer: &mut dyn Write) -> io::Result<()> {
    loop {
        let mut byte = (n & 0x7F) as u8;
        n >>= 7;
        if n != 0 {
            byte |= 0x80;
        }
        writer.write_all(&[byte])?;
        if n == 0 {
            break;
        }
    }
    Ok(())
}

/// Сериализация Option<serde_json::Value> в json-строку с длиной.
fn write_json_value(val: &Option<serde_json::Value>, writer: &mut dyn Write) -> io::Result<()> {
    if let Some(json) = val {
        let s = serde_json::to_string(json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        write_varint(s.len() as u64, writer)?;
        writer.write_all(s.as_bytes())?;
    } else {
        write_varint(0, writer)?;
    }
    Ok(())
}

/// Сериализация Option<SynType> через json-строку.
fn write_syn_type(ty: &Option<SynType>, writer: &mut dyn Write) -> io::Result<()> {
    if let Some(t) = ty {
        let val = serde_json::to_value(t)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        write_json_value(&Some(val), writer)
    } else {
        write_varint(0, writer)
    }
}

/// Запись графа ASG в файл в формате SYN1.
pub fn save_syn1<P: AsRef<std::path::Path>>(asg: &ASG, path: P) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // 1. Узлы
    write_varint(asg.nodes.len() as u64, &mut writer)?;
    for node in &asg.nodes {
        write_varint(node.id, &mut writer)?;
        write_varint(node.code as u64, &mut writer)?;
        write_json_value(&node.value, &mut writer)?;
        write_syn_type(&node.ty, &mut writer)?;
    }

    // 2. Ребра
    write_varint(asg.edges.len() as u64, &mut writer)?;
    for edge in &asg.edges {
        write_varint(edge.from, &mut writer)?;
        write_varint(edge.to, &mut writer)?;
        write_varint(edge.code as u64, &mut writer)?;
    }

    writer.flush()?;
    Ok(())
}
