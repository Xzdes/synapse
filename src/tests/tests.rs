//! Юнит-тесты для Synapse.

use synapse::asg::ASG;
use synapse::node_factories::literal_int;
use synapse::interpreter::InterpreterContext;
use synapse::{
    llvm_backend::LLVMBackend,
    wasm_backend::WasmBackend,
    c_backend::CBackend,
    js_backend::JsBackend,
    proof::ProofDSL,
    type_checker,
    tools::graphviz_exporter,
};

#[test]
fn test_interpreter_runs() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}

#[test]
fn test_llvm_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let ir = LLVMBackend::compile(&asg).unwrap();
    assert!(ir.contains("LLVM IR"));
}

#[test]
fn test_wasm_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let wasm = WasmBackend::compile(&asg).unwrap();
    assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6D])); // 'asm'
}

#[test]
fn test_c_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let c_code = CBackend::generate_c(&asg).unwrap();
    assert!(c_code.contains("int main"));
}

#[test]
fn test_js_backend() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let js_code = JsBackend::generate_js(&asg).unwrap();
    assert!(js_code.contains("console.log"));
}

#[test]
fn test_proof_dsl() {
    let proof = ProofDSL::new();
    proof.add_proof("Test Proof").unwrap();
    proof.add_assertion("Test Assertion").unwrap();
    proof.add_assumption("Test Assumption").unwrap();
    proof.add_specification("Test Specification").unwrap();
}

#[test]
fn test_type_checker() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    type_checker::check_types(&asg).unwrap();
    type_checker::infer_types(&asg).unwrap();
}

#[test]
fn test_graphviz_exporter() {
    let mut asg = ASG::new();
    asg.add_node(literal_int(1, 42));
    let path = "test_output.dot";
    graphviz_exporter::export_to_dot(&asg, path).unwrap();
    assert!(std::path::Path::new(path).exists());
    std::fs::remove_file(path).unwrap();
}
