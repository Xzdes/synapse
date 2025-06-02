// src/tools/generate_gt_if.rs

//! Генератор: сравнение двух чисел (a > b), печать результата.
//! if a > b { print(1) } else { print(0) }

use synapse::asg::{ASG, Edge, Node};
use synapse::node_factories;
use synapse::nodecodes::NodeType;
use synapse::types::SynType;
use synapse::syn1_writer::save_syn1;

fn main() {
    // 1. Литералы для сравнения: a = 7, b = 5
    let n1 = node_factories::literal_int(1, 7); // a
    let n2 = node_factories::literal_int(2, 5); // b

    // 2. Узел сравнения (a > b) — возвращает 1 (true) или 0 (false)
    let n3 = Node {
        id: 3,
        code: NodeType::Gt as u16,
        value: Some(serde_json::json!({
            "left": 1,
            "right": 2
        })),
        ty: Some(SynType::Bool),
    };

    // 3. then и else
    let n4 = node_factories::literal_int(4, 1); // then_val
    let n5 = node_factories::literal_int(5, 0); // else_val

    // 4. PatternMatch: if n3 { n4 } else { n5 }
    let n6 = Node {
        id: 6,
        code: NodeType::PatternMatch as u16,
        value: Some(serde_json::json!({
            "cond": 3,
            "then": 4,
            "else": 5
        })),
        ty: Some(SynType::Int),
    };

    // 5. Print результата
    let n7 = node_factories::print(7, 6);

    // 6. Граф и связи
    let nodes = vec![n1, n2, n3, n4, n5, n6, n7];
    let edges = vec![
        Edge { from: 1, to: 3, code: 0 },
        Edge { from: 2, to: 3, code: 0 },
        Edge { from: 3, to: 6, code: 0 },
        Edge { from: 4, to: 6, code: 0 },
        Edge { from: 5, to: 6, code: 0 },
        Edge { from: 6, to: 7, code: 0 },
    ];

    let asg = ASG { nodes, edges };

    // 7. Сохраняем
    save_syn1(&asg, "gt_if.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: gt_if.synapse");
}
