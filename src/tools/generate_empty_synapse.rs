// src/tools/generate_empty_synapse.rs

//! Генератор: создает пустой (или почти пустой) Synapse-граф.

use synapse::asg::ASG;
use synapse::syn1_writer::save_syn1;

fn main() {
    // Просто пустой список узлов и ребер
    let asg = ASG {
        nodes: vec![],
        edges: vec![],
    };

    save_syn1(&asg, "empty_synapse.synapse").expect("Failed to save SYN1 file");
    println!("Граф успешно сгенерирован: empty_synapse.synapse");
}
