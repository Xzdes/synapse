// src/syn1.rs

//! Загрузка ASG-графа Synapse из бинарного файла в формате SYN1.
//! Формат очень простой: [кол-во узлов][узлы][кол-во ребер][ребра]
//! Каждый узел: [id][code][json value][json ty]
//! Каждый ребро: [from][to][code]

use std::fs::File;
use std::io::{self, Read, BufReader};
use crate::asg::{Node, Edge, ASG};
use crate::types::SynType;

/// Читает VarInt с файла (для совместимости с компактным форматом).
fn read_varint<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut result = 0u64;
    let mut shift = 0u8;
    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

/// Читает JSON-значение (value или ty) из файла как строку, потом парсит.
fn read_json_value<R: Read>(reader: &mut R) -> io::Result<Option<serde_json::Value>> {
    let len = read_varint(reader)? as usize;
    if len == 0 {
        return Ok(None);
    }
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    let s = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let val = serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(val))
}

/// Читает граф из бинарного файла SYN1.
pub fn load_syn1<P: AsRef<std::path::Path>>(path: P) -> io::Result<ASG> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // 1. Узлы
    let node_count = read_varint(&mut reader)? as usize;
    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        let id = read_varint(&mut reader)?;
        let code = read_varint(&mut reader)? as u16;
        let value = read_json_value(&mut reader)?;
        // Для типа: сериализуем SynType как json-строку (или None)
        let ty_val = read_json_value(&mut reader)?;
        let ty = if let Some(t) = ty_val {
            Some(serde_json::from_value::<SynType>(t)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
        } else {
            None
        };
        nodes.push(Node { id, code, value, ty });
    }

    // 2. Ребра
    let edge_count = read_varint(&mut reader)? as usize;
    let mut edges = Vec::with_capacity(edge_count);
    for _ in 0..edge_count {
        let from = read_varint(&mut reader)?;
        let to = read_varint(&mut reader)?;
        let code = read_varint(&mut reader)? as u16;
        edges.push(Edge { from, to, code });
    }

    Ok(ASG { nodes, edges })
}
