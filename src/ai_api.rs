//! Модуль `ai_api`
//!
//! API для интеграции Synapse с AI-инструментами:
//! - JSON-сериализация/десериализация
//! - Аннотации ASG (заглушка для будущих метаданных)

use serde::{Deserialize, Serialize};
use serde_json;

use crate::asg::ASG;
use crate::{SynapseError, SynapseResult};

/// Сериализовать ASG в JSON.
pub fn asg_to_json(asg: &ASG) -> SynapseResult<String> {
    serde_json::to_string_pretty(asg)
        .map_err(|e| SynapseError::Serialization(format!("Failed to serialize ASG to JSON: {}", e)))
}

/// Десериализовать ASG из JSON.
pub fn asg_from_json(json: &str) -> SynapseResult<ASG> {
    serde_json::from_str(json)
        .map_err(|e| SynapseError::Serialization(format!("Failed to deserialize ASG from JSON: {}", e)))
}

/// Заглушка для аннотаций (metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsgAnnotation {
    /// Имя аннотации.
    pub name: String,
    /// Значение аннотации.
    pub value: String,
}

/// Добавить аннотацию к ASG.
/// (На данном этапе — заглушка, без реальной интеграции.)
pub fn add_annotation(_asg: &mut ASG, annotation: AsgAnnotation) {
    println!("Adding annotation: {} = {}", annotation.name, annotation.value);
    // В будущем: добавить поле аннотаций в ASG.
}
