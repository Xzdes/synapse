// src/tools/generate_float_sub.rs

//! Генератор: вычитает два литеральных float и сохраняет результат в графе.

use synapse::asg::{ASG, Edge};
use synapse::node_factories;
use synapse::syn1_writer::save_syn1;

fn main() {
    // 1. Литералы 5.5 и 2.25
    let n1 = node_factories::literal_float(1, 5.5);
    let n2 = node_factories::literal_float(2, 2.25);

    // 2. Вычитание: n3 = n1 - n2
    let n3 = node_factories::binary_sub(3, 1, 2);

    // 3. Print: вывести n3
    let n4 = node_factories::print(4, 3);

    // 4. Граф и связи
    let nodes = vec![n1, n2, n3, n4];
    let edges = vec![
        Edge { from: 1, to: 3, code: 0 },
        Edge { from: 2, to: 3, code: 0 },
        Edge { from: 3, to: 4, code: 0 },
    ];

    let asg = ASG { nodes, edges };

    // 5. Сохраняем
    save_syn1(&asg, "float_sub.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: float_sub.synapse");
}
