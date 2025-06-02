// src/lib.rs

//! Главная точка входа для библиотеки Synapse.
//! Здесь экспортируются все основные модули.

pub mod asg;           // Структуры ASG: Node, Edge, ASG
pub mod nodecodes;     // Типы узлов и ребер (enum NodeType, EdgeType)
pub mod types;         // Типовая система (SynType) и ошибки (SynError)
pub mod syn1;          // Загрузка бинарного SYN1-формата
pub mod syn1_writer;   // Сохранение в бинарный SYN1
pub mod interpreter;   // Интерпретатор ASG
pub mod node_factories;
// Другие модули (если есть): например, для генераторов, CLI и др.
