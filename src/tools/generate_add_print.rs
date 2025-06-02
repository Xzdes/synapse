// src/tools/generate_add_print.rs

//! Пример генератора: создает граф для сложения двух чисел и печати результата.

use synapse::asg::{ASG, Edge};
use synapse::node_factories;
use synapse::syn1_writer::save_syn1;

fn main() {
    // 1. Литералы (1 и 2)
    let n1 = node_factories::literal_int(1, 1);
    let n2 = node_factories::literal_int(2, 2);

    // 2. Узел сложения: n3 = n1 + n2
    let n3 = node_factories::binary_add(3, 1, 2);

    // 3. Print: вывести n3
    let n4 = node_factories::print(4, 3);

    // 4. Граф (связи минимальные: каждый next к следующему)
    let nodes = vec![n1, n2, n3, n4];
    let edges = vec![
        Edge { from: 1, to: 3, code: 0 }, // n1 -> n3 (не обязательно для данного генератора, но можно расширять)
        Edge { from: 2, to: 3, code: 0 }, // n2 -> n3
        Edge { from: 3, to: 4, code: 0 }, // n3 -> n4
    ];

    let asg = ASG { nodes, edges };

    // 5. Сохраняем граф в файл (add_print.synapse)
    save_syn1(&asg, "add_print.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: add_print.synapse");
}
