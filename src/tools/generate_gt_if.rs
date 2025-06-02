use synapse::asg::{ASG, Node};
use synapse::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // Пример: if (a > b) { print(1) } else { print(0) }
    let n1 = Node::literal_int(1, 7);         // a = 7
    let n2 = Node::literal_int(2, 5);         // b = 5
    let n3 = Node::binary_gt(3, 1, 2);        // n3 = a > b ? (1 если да, 0 если нет)
    let n4 = Node::literal_int(4, 1);         // then_val
    let n5 = Node::literal_int(5, 0);         // else_val
    let n6 = Node::conditional(6, 3, 4, 5);   // if n3 != 0 then n4 else n5
    let n7 = Node::perform_io_write_line(7, 6);// print(result)

    let asg = ASG {
        nodes: vec![n1, n2, n3, n4, n5, n6, n7],
        entry: 7,
    };

    save_synapse_file("gt_if.synapse", &asg)?;
    println!("Файл gt_if.synapse успешно создан!");
    Ok(())
}
