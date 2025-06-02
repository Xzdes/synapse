use crate::asg::{ASG, Node, Edge};
use anyhow::{Result, bail};
use byteorder::{ReadBytesExt};
use std::fs::File;
use std::io::{Read, Cursor};

/// Load SYN1 file and deserialize it into ASG
pub fn load_synapse_file(path: &str) -> Result<ASG> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut cursor = Cursor::new(buffer);

    // Read SYN1 header (first 4 bytes)
    let mut header = [0u8; 4];
    cursor.read_exact(&mut header)?;
    if &header != b"SYN1" {
        bail!("Invalid SYN1 header");
    }

    // Read Version
    let version = read_varint(&mut cursor)?;
    if version != 1 {
        bail!("Unsupported SYN1 version");
    }

    // Read node count
    let node_count = read_varint(&mut cursor)?;

    let mut nodes = Vec::new();
    for _ in 0..node_count {
        let node_type_code = read_varint(&mut cursor)?;
        let node_id = read_varint(&mut cursor)?;
        let payload_len = read_varint(&mut cursor)?;
        let mut payload = vec![0u8; payload_len as usize];
        cursor.read_exact(&mut payload)?;

        let edge_count = read_varint(&mut cursor)?;
        let mut edges = Vec::new();
        for _ in 0..edge_count {
            let edge_type_code = read_varint(&mut cursor)?;
            let target_node_id = read_varint(&mut cursor)?;
            let edge_payload_len = read_varint(&mut cursor)?;
            let edge_payload = if edge_payload_len > 0 {
                let mut buf = vec![0u8; edge_payload_len as usize];
                cursor.read_exact(&mut buf)?;
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

    // Entry point пока 0 (будет доработано)
    Ok(ASG { nodes, entry: 0 })
}

/// Helper: Read VarInt (little-endian)
fn read_varint(cursor: &mut Cursor<Vec<u8>>) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    loop {
        let byte = cursor.read_u8()?;
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}
