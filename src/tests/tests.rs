//! Юнит-тесты для Synapse.
//!
//! Тестируем:
//! - TypeChecker (check_types, infer_types).
//! - Proof SMT (ProofDSL).
//! - Бэкенды (LLVM, C, JS, WASM).

use synapse::asg::{ASG, NodeID};
use synapse::node_factories::literal_int;
use synapse::interpreter::InterpreterContext;
use synapse::{
    type_checker,
    proof,
    proof_dsl::ProofDSL,
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
    let mut proof_dsl = ProofDSL::new();
    assert!(proof_dsl.assert("(declare-const x Int)").is_ok());
    assert!(proof_dsl.assert("(assert (> x 0))").is_ok());
    let result = proof_dsl.check().unwrap();
    assert!(result);
}

#[test]
fn test_check_proofs() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let result = proof::check_proofs(&asg).unwrap();
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
