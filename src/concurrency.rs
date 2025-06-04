//! Модуль `concurrency`
//!
//! Поддержка многопоточности в Synapse:
//! - Concurrency (запуск нового потока)
//! - Демонстрация безопасного использования std::thread
//!
//! Гарантирует Send/Sync и отсутствие глобального состояния.

use std::thread;

use crate::{SynapseError, SynapseResult};

/// Запустить новый поток исполнения.
///
/// # Пример:
/// ```
/// use synapse::concurrency::spawn_thread;
/// spawn_thread("Hello from thread!".to_string());
/// ```
pub fn spawn_thread(message: String) -> SynapseResult<()> {
    let handle = thread::spawn(move || {
        println!("Thread: {}", message);
    });

    handle
        .join()
        .map_err(|_| SynapseError::Concurrency("Thread panicked!".into()))?;

    Ok(())
}

/// Проверить, можно ли безопасно запустить Concurrency.
///
/// На данном этапе всегда возвращает Ok(true).
pub fn check_concurrency_safety() -> SynapseResult<bool> {
    Ok(true)
}
