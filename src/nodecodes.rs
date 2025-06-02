// src/nodecodes.rs

use serde::{Serialize, Deserialize};

/// Типы узлов (NodeType) в Synapse ASG.
/// Определяют роль узла в графе.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    // Арифметика и логика
    Add,        // сложение
    Sub,        // вычитание
    Mul,        // умножение
    Div,        // деление
    And,        // логическое И
    Or,         // логическое ИЛИ
    Not,        // логическое НЕ
    Gt,         // больше
    Lt,         // меньше
    Eq,         // равно

    // Прочее
    Print,      // вывод значения
    Const,      // литерал (константа)

    // Новые типы для расширения
    Variable,      // переменная (объявление/использование)
    Function,      // функция (определение или вызов)
    Effect,        // побочный эффект (например, I/O)
    PatternMatch,  // pattern matching (основа для match-case)
    FFI,           // foreign function interface (внешний вызов)

    // В будущем: Return, Assign, Block, Call и др.
}

/// Типы ребер (EdgeType) в Synapse ASG.
/// Определяют связь между узлами.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    Next,        // последовательность (переход к следующему)
    Data,        // передача данных (например, аргумент)
    Condition,   // условная ветка
    TrueBranch,  // переход по true
    FalseBranch, // переход по false

    // Можно расширять новыми типами связей.
}
