use synapse_project::asg::{ASG, Node};
use synapse_project::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // Граф: x = 1 + 2
    let n1 = Node::literal_int(1, 1);
    let n2 = Node::literal_int(2, 2);
    let n3 = Node::binary_add(3, 1, 2);

    let asg = ASG {
        nodes: vec![n1, n2, n3],
        entry: 3,
    };

    save_synapse_file("literal_add.synapse", &asg)?;
    println!("Файл literal_add.synapse успешно создан!");
    Ok(())
}
