use synapse::asg::{ASG, Node};
use synapse::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // x = 1 + 2; print(x)
    let n1 = Node::literal_int(1, 1);
    let n2 = Node::literal_int(2, 2);
    let n3 = Node::binary_add(3, 1, 2);
    let n4 = Node::perform_io_write_line(4, 3);

    let asg = ASG {
        nodes: vec![n1, n2, n3, n4],
        entry: 4,
    };

    save_synapse_file("add_print.synapse", &asg)?;
    println!("Файл add_print.synapse успешно создан!");
    Ok(())
}
