// --- ИСПРАВЛЕНИЕ: Убираем неиспользуемый `NodeID` ---
use synapse::asg::{ASG, Edge, Node};
use synapse::nodecodes::{NodeType, EdgeType};
use synapse::interpreter::{Interpreter, Value};

fn main() {
    println!("--- Synapse Example: Executing 5 + 8 ---");

    let mut asg = ASG::new();

    asg.add_node(Node {
        id: 1,
        node_type: NodeType::LiteralInt,
        payload: Some(5i64.to_le_bytes().to_vec()),
        edges: vec![],
    });

    asg.add_node(Node {
        id: 2,
        node_type: NodeType::LiteralInt,
        payload: Some(8i64.to_le_bytes().to_vec()),
        edges: vec![],
    });

    asg.add_node(Node {
        id: 3,
        node_type: NodeType::BinaryOperation,
        payload: None,
        edges: vec![
            Edge { edge_type: EdgeType::ApplicationArgument, target_node_id: 1, payload: None },
            Edge { edge_type: EdgeType::ApplicationArgument, target_node_id: 2, payload: None },
        ],
    });

    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&asg, 3);

    // --- ИСПРАВЛЕНИЕ: Упрощаем `match`, убирая недостижимую ветку ---
    match result {
        Ok(Value::Int(val)) => {
            println!("\nExecution successful!");
            println!("Result: {}", val);
            assert_eq!(val, 13);
        }
        Err(e) => {
            eprintln!("\nExecution failed: {}", e);
        }
    }
}