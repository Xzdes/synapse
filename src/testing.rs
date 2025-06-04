//! Модуль `testing`
//!
//! Тестирование в Synapse:
//! - TestCase, TestSuite, Assertions
//!
//! Тесты встраиваются в ASG как узлы, а также доступны через API.

use serde::{Deserialize, Serialize};

use crate::SynapseResult;

/// Тест-кейс.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// Имя теста.
    pub name: String,
    /// Описание.
    pub description: String,
    /// Проверяемая функция (или идентификатор).
    pub function: String,
}

/// Набор тестов.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Имя набора.
    pub name: String,
    /// Список тестов.
    pub tests: Vec<TestCase>,
}

/// Выполнить тест-кейс (заглушка).
pub fn run_test_case(test: &TestCase) -> SynapseResult<()> {
    println!("Running test `{}`...", test.name);
    // В будущем: исполнение через интерпретатор.
    Ok(())
}

/// Выполнить набор тестов.
pub fn run_test_suite(suite: &TestSuite) -> SynapseResult<()> {
    println!("Running test suite `{}`...", suite.name);
    for test in &suite.tests {
        run_test_case(test)?;
    }
    Ok(())
}
