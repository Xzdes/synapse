use synapse::asg::{ASG, Node};
use synapse::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // Пример: x = 5.5 - 2.25; print(x)
    let n1 = Node::literal_float(1, 5.5);
    let n2 = Node::literal_float(2, 2.25);
    let n3 = Node::binary_sub(3, 1, 2);
    let n4 = Node::perform_io_write_line(4, 3);

    let asg = ASG {
        nodes: vec![n1, n2, n3, n4],
        entry: 4,
    };

    save_synapse_file("float_sub.synapse", &asg)?;
    println!("Файл float_sub.synapse успешно создан!");
    Ok(())
}
