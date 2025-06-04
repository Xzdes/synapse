//! Модуль `graphviz_exporter`
//!
//! Экспорт ASG в формат DOT для визуализации с помощью GraphViz.
//!
//! Планируется поддержка:
//! - Генерация графа из ASG.
//! - Настройка стилей узлов и рёбер.
//! - Экспорт в файл .dot для последующего рендеринга.

use crate::asg::{ASG, Node, Edge};
use crate::{SynapseError, SynapseResult};

use std::fs::File;
use std::io::Write;

/// Экспортировать ASG в файл формата DOT.
pub fn export_to_dot(asg: &ASG, path: &str) -> SynapseResult<()> {
    let mut file = File::create(path)
        .map_err(|e| SynapseError::Serialization(format!("Failed to create file: {}", e)))?;

    writeln!(file, "digraph ASG {{")
        .map_err(|e| SynapseError::Serialization(format!("Failed to write DOT header: {}", e)))?;

    // Узлы
    for node in &asg.nodes {
        writeln!(
            file,
            "    node{} [label=\"{}\"]",
            node.id, format!("{:?}", node.node_type)
        )
        .map_err(|e| SynapseError::Serialization(format!("Failed to write node: {}", e)))?;
    }

    // Рёбра
    for node in &asg.nodes {
        for edge in &node.edges {
            writeln!(
                file,
                "    node{} -> node{} [label=\"{:?}\"]",
                node.id, edge.target_node_id, edge.edge_type
            )
            .map_err(|e| SynapseError::Serialization(format!("Failed to write edge: {}", e)))?;
        }
    }

    writeln!(file, "}}")
        .map_err(|e| SynapseError::Serialization(format!("Failed to write DOT footer: {}", e)))?;

    println!("GraphvizExporter: exported ASG to '{}'.", path);
    Ok(())
}
