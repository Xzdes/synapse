//! Модуль тестов для Synapse.
//!
//! Проверяет:
//! - Сериализацию/десериализацию ASG (SYN1)
//! - JSON конвертеры
//! - Интерпретатор (базовые вызовы)
//! - Эффекты
//! - Proof
//! - Макросы

use synapse::{
    ai_api::{asg_from_json, asg_to_json},
    asg::{ASG, Node},
    effects::{perform_console_output, perform_io},
    interpreter::InterpreterContext,
    node_factories::{literal_int, binary_operation},
    proof::{check_assert, check_assume},
    syn1::{load_syn1},
    syn1_writer::save_syn1,
    SynapseResult,
};

#[test]
fn test_serialization_and_deserialization() -> SynapseResult<()> {
    let mut asg = ASG::new();
    let node1 = literal_int(1, 42);
    let node2 = binary_operation(2, 1);
    asg.add_node(node1);
    asg.add_node(node2);

    save_syn1(&asg, "test_graph.synapse")?;
    let loaded_asg = load_syn1("test_graph.synapse")?;

    assert_eq!(asg.nodes.len(), loaded_asg.nodes.len());
    Ok(())
}

#[test]
fn test_json_serialization() -> SynapseResult<()> {
    let mut asg = ASG::new();
    let node1 = literal_int(1, 100);
    asg.add_node(node1);

    let json = asg_to_json(&asg)?;
    let restored_asg = asg_from_json(&json)?;

    assert_eq!(asg.nodes.len(), restored_asg.nodes.len());
    Ok(())
}

#[test]
fn test_interpreter_execution() -> SynapseResult<()> {
    let mut asg = ASG::new();
    let node = literal_int(1, 7);
    asg.add_node(node);

    let interpreter = InterpreterContext;
    interpreter.execute(&asg)?;
    Ok(())
}

#[test]
fn test_effects() -> SynapseResult<()> {
    perform_console_output("Hello from test!")?;
    let echoed = perform_io("Echo test")?;
    assert_eq!(echoed, "Echo test");
    Ok(())
}

#[test]
fn test_proof_check() -> SynapseResult<()> {
    check_assert(true, "This should not fail")?;
    check_assume(false, "This should warn but not fail")?;
    Ok(())
}
