//! Определения кодов для узлов и ребер.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    LiteralInt,
    BinaryOperation, // Для целочисленной арифметики
    LiteralTensor,   // Новый тип для тензорных литералов
    TensorAdd,       // Новый тип для сложения тензоров
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    ApplicationArgument,
}