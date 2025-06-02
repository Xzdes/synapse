// src/tools/generate_conditional.rs

//! Генератор: пример простого условия (if) для Synapse-графа.
//! if cond != 0 { print(then_val) } else { print(else_val) }

use synapse::asg::{ASG, Edge};
use synapse::node_factories;
use synapse::syn1_writer::save_syn1;

fn main() {
    // 1. Условие (например, 1 - истина)
    let n1 = node_factories::literal_int(1, 1);

    // 2. Значения для then и else
    let n2 = node_factories::literal_int(2, 42);  // then_val
    let n3 = node_factories::literal_int(3, 99);  // else_val

    // 3. Узел: n4 = n1 ? n2 : n3 (имитируем "if" через структуру value)
    // Можно сделать собственный тип узла PatternMatch, но для примера просто пишем структуру.
    let n4 = synapse::asg::Node {
        id: 4,
        code: synapse::nodecodes::NodeType::PatternMatch as u16,
        value: Some(serde_json::json!({
            "cond": 1,
            "then": 2,
            "else": 3
        })),
        ty: Some(synapse::types::SynType::Int),
    };

    // 4. Print результата
    let n5 = node_factories::print(5, 4);

    // 5. Граф и связи
    let nodes = vec![n1, n2, n3, n4, n5];
    let edges = vec![
        Edge { from: 1, to: 4, code: 0 },
        Edge { from: 2, to: 4, code: 0 },
        Edge { from: 3, to: 4, code: 0 },
        Edge { from: 4, to: 5, code: 0 },
    ];

    let asg = ASG { nodes, edges };

    // 6. Сохраняем
    save_syn1(&asg, "conditional.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: conditional.synapse");
}
