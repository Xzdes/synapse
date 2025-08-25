use synapse::asg::{ASG, Edge, Node};
use synapse::nodecodes::{NodeType, EdgeType};
use synapse::interpreter::{Interpreter, Value};

fn main() {
    println!("--- Synapse: Autograd Demo ---");
    println!("--- Executing: Tensor(2.0) + Tensor(5.0) ---");

    let mut asg = ASG::new();

    // Литерал-тензор со значением 2.0
    asg.add_node(Node {
        id: 1,
        node_type: NodeType::LiteralTensor,
        payload: Some(2.0f32.to_le_bytes().to_vec()),
        edges: vec![],
    });

    // Литерал-тензор со значением 5.0
    asg.add_node(Node {
        id: 2,
        node_type: NodeType::LiteralTensor,
        payload: Some(5.0f32.to_le_bytes().to_vec()),
        edges: vec![],
    });

    // Узел сложения тензоров
    asg.add_node(Node {
        id: 3,
        node_type: NodeType::TensorAdd,
        payload: None,
        edges: vec![
            Edge { edge_type: EdgeType::ApplicationArgument, target_node_id: 1, payload: None },
            Edge { edge_type: EdgeType::ApplicationArgument, target_node_id: 2, payload: None },
        ],
    });

    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&asg, 3);

    match result {
        Ok(Value::Tensor(t)) => {
            println!("\nExecution successful!");
            let result_val = t.data.borrow().sum();
            println!("Result: Tensor({})", result_val);
            assert_eq!(result_val, 7.0);

            println!("\nRunning backward pass...");
            t.backward();

            // --- ИСПРАВЛЕНИЕ: Заимствуем, а не перемещаем ---
            let inputs = &t.ctx.as_ref().unwrap().inputs;
            let grad_a = inputs[0].grad.as_ref().unwrap().borrow().sum();
            let grad_b = inputs[1].grad.as_ref().unwrap().borrow().sum();
            println!("Gradient of first input: {}", grad_a);
            println!("Gradient of second input: {}", grad_b);
            assert_eq!(grad_a, 1.0);
            assert_eq!(grad_b, 1.0);
            println!("\nGradients are correct!");
        }
        Ok(other) => println!("\nExecution finished with unexpected value: {:?}", other),
        Err(e) => eprintln!("\nExecution failed: {}", e),
    }
}