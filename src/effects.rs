//! Модуль `effects`
//!
//! Реализация эффектов в Synapse:
//! - EffectIO, EffectConsole, EffectFSRead, EffectFSWrite
//! - Runtime API (без unsafe, через std::fs и std::io)
//!
//! Все ошибки через SynapseError::Effect.

use std::fs::File;
use std::io::{Read, Write};

use crate::{SynapseError, SynapseResult};

/// Эффект: ввод/вывод (IO).
pub fn perform_io(input: &str) -> SynapseResult<String> {
    println!("EffectIO: {}", input);
    Ok(input.to_string())
}

/// Эффект: вывод в консоль.
pub fn perform_console_output(message: &str) -> SynapseResult<()> {
    println!("EffectConsole: {}", message);
    Ok(())
}

/// Эффект: чтение из файла.
pub fn perform_fs_read(path: &str) -> SynapseResult<String> {
    let mut file = File::open(path)
        .map_err(|e| SynapseError::Effect(format!("Failed to open file: {}", e)))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| SynapseError::Effect(format!("Failed to read file: {}", e)))?;
    Ok(contents)
}

/// Эффект: запись в файл.
pub fn perform_fs_write(path: &str, data: &str) -> SynapseResult<()> {
    let mut file = File::create(path)
        .map_err(|e| SynapseError::Effect(format!("Failed to create file: {}", e)))?;
    file.write_all(data.as_bytes())
        .map_err(|e| SynapseError::Effect(format!("Failed to write file: {}", e)))?;
    Ok(())
}
