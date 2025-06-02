// src/tools/generate_literal_add.rs

//! Генератор: складывает два литеральных числа и сохраняет результат в графе.

use synapse::asg::{ASG, Edge};
use synapse::node_factories;
use synapse::syn1_writer::save_syn1;

fn main() {
    // 1. Литералы 1 и 2
    let n1 = node_factories::literal_int(1, 1);
    let n2 = node_factories::literal_int(2, 2);

    // 2. Сложение: n3 = n1 + n2
    let n3 = node_factories::binary_add(3, 1, 2);

    // 3. Print: вывести n3
    let n4 = node_factories::print(4, 3);

    // 4. Граф и простые связи (можно обойтись без них, но оставим для наглядности)
    let nodes = vec![n1, n2, n3, n4];
    let edges = vec![
        Edge { from: 1, to: 3, code: 0 },
        Edge { from: 2, to: 3, code: 0 },
        Edge { from: 3, to: 4, code: 0 },
    ];

    let asg = ASG { nodes, edges };

    // 5. Сохраняем
    save_syn1(&asg, "literal_add.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: literal_add.synapse");
}
