//! Модуль `ai_api`
//!
//! Интерфейс для взаимодействия с ИИ или внешними инструментами.
//!
//! Поддерживает:
//! - Импорт/экспорт ASG в JSON.
//! - Чтение/запись графа программ.
//! - Интеграцию с AI-инструментами для анализа кода.

use crate::asg::ASG;
use crate::{SynapseError, SynapseResult};
use serde_json;

/// Экспортировать ASG в JSON-строку.
pub fn export_asg_to_json(asg: &ASG) -> SynapseResult<String> {
    serde_json::to_string_pretty(asg)
        .map_err(|e| SynapseError::Serialization(format!("Failed to export ASG to JSON: {}", e)))
}

/// Импортировать ASG из JSON-строки.
pub fn import_asg_from_json(json_str: &str) -> SynapseResult<ASG> {
    serde_json::from_str(json_str)
        .map_err(|e| SynapseError::Serialization(format!("Failed to import ASG from JSON: {}", e)))
}

/// Сохранить ASG в файл JSON.
pub fn save_asg_to_file(asg: &ASG, path: &str) -> SynapseResult<()> {
    let json = export_asg_to_json(asg)?;
    std::fs::write(path, json)
        .map_err(|e| SynapseError::Serialization(format!("Failed to write ASG to file: {}", e)))
}

/// Загрузить ASG из файла JSON.
pub fn load_asg_from_file(path: &str) -> SynapseResult<ASG> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| SynapseError::Serialization(format!("Failed to read ASG from file: {}", e)))?;
    import_asg_from_json(&json)
}
