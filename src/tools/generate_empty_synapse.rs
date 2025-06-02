use std::fs::File;
use std::io::{Write};
use anyhow::Result;

fn main() -> Result<()> {
    let mut file = File::create("empty.synapse")?;

    // Заголовок "SYN1"
    file.write_all(b"SYN1")?;

    // Версия (VarInt)
    write_varint(&mut file, 1)?;

    // Количество узлов (VarInt)
    write_varint(&mut file, 0)?;

    println!("Empty .synapse file 'empty.synapse' created successfully.");
    Ok(())
}

// Простейшая сериализация VarInt
fn write_varint(file: &mut File, mut value: u64) -> Result<()> {
    while value >= 0x80 {
        file.write_all(&[((value as u8) & 0x7F) | 0x80])?;
        value >>= 7;
    }
    file.write_all(&[value as u8])?;
    Ok(())
}
