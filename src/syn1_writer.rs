//! Модуль `syn1_writer`
//!
//! Запись бинарного формата SYN1 (ASG):
//! - Little Endian, VarInt
//! - Поддержка всех узлов/рёбер, включая payload
//!
//! Ошибки обрабатываются через SynapseError::Serialization.

use std::fs::File;
use std::io::{Write, BufWriter};
use byteorder::WriteBytesExt;

use crate::asg::ASG;
use crate::{SynapseError, SynapseResult};

/// Сохранение ASG в бинарный формат SYN1.
pub fn save_syn1(asg: &ASG, path: &str) -> SynapseResult<()> {
    let file = File::create(path)
        .map_err(|e| SynapseError::Serialization(format!("Failed to create file: {}", e)))?;
    let mut writer = BufWriter::new(file);

    // Заголовок: "SYN1"
    writer
        .write_all(b"SYN1")
        .map_err(|e| SynapseError::Serialization(format!("Failed to write header: {}", e)))?;

    // Версия (u8)
    writer
        .write_u8(1)
        .map_err(|e| SynapseError::Serialization(format!("Failed to write version: {}", e)))?;

    // Количество узлов (VarInt)
    write_varint(&mut writer, asg.nodes.len() as u64)?;

    // Сохраняем каждый узел
    for node in &asg.nodes {
        write_varint(&mut writer, node.node_type as u64)?;

        write_varint(&mut writer, node.id)?;

        if let Some(payload) = &node.payload {
            write_varint(&mut writer, payload.len() as u64)?;
            writer
                .write_all(payload)
                .map_err(|e| SynapseError::Serialization(format!("Failed to write node payload: {}", e)))?;
        } else {
            write_varint(&mut writer, 0)?;
        }

        write_varint(&mut writer, node.edges.len() as u64)?;

        for edge in &node.edges {
            write_varint(&mut writer, edge.edge_type as u64)?;

            write_varint(&mut writer, edge.target_node_id)?;

            if let Some(edge_payload) = &edge.payload {
                write_varint(&mut writer, edge_payload.len() as u64)?;
                writer
                    .write_all(edge_payload)
                    .map_err(|e| SynapseError::Serialization(format!("Failed to write edge payload: {}", e)))?;
            } else {
                write_varint(&mut writer, 0)?;
            }
        }
    }

    writer.flush()
        .map_err(|e| SynapseError::Serialization(format!("Failed to flush writer: {}", e)))?;

    Ok(())
}

/// Запись VarInt в поток.
fn write_varint<W: Write>(writer: &mut W, mut value: u64) -> SynapseResult<()> {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        writer
            .write_u8(byte)
            .map_err(|e| SynapseError::Serialization(format!("Failed to write varint: {}", e)))?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}
