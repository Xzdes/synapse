//! Юнит-тесты для Synapse.
//!
//! Тестируем:
//! - TypeChecker (check_types, infer_types).
//! - ProofSMT (solve_proof).
//! - Бэкенды (LLVM, C, JS, WASM).

use synapse::asg::{ASG, NodeID};
use synapse::node_factories::literal_int;
use synapse::interpreter::InterpreterContext;
use synapse::{
    type_checker,
    proof_smt,
    llvm_backend::LLVMBackend,
    c_backend::CBackend,
    js_backend::JsBackend,
    wasm_backend::WasmBackend,
};

#[test]
fn test_type_checker() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    assert!(type_checker::check_types(&asg).is_ok());
    assert!(type_checker::infer_types(&asg).is_ok());
}

#[test]
fn test_proof_smt() {
    let result = proof_smt::solve_proof("x > 0").unwrap();
    assert!(result);
}

#[test]
fn test_llvm_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let ir = LLVMBackend::compile(&asg).unwrap();
    assert!(ir.contains("ModuleID"));
}

#[test]
fn test_c_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let c_code = CBackend::generate_c(&asg).unwrap();
    assert!(c_code.contains("printf"));
}

#[test]
fn test_js_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let js_code = JsBackend::generate_js(&asg).unwrap();
    assert!(js_code.contains("console.log"));
}

#[test]
fn test_wasm_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let wasm = WasmBackend::compile(&asg).unwrap();
    assert_eq!(wasm[..4], [0x00, 0x61, 0x73, 0x6D]);
}
