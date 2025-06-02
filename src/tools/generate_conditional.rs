use synapse::asg::{ASG, Node};
use synapse::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // Пример: if (cond) { then_val } else { else_val } => print(result)
    // Меняй cond_value для теста (1 или 0)
    let cond_value: i64 = 1; // попробуй 0 для else-ветки

    let n1 = Node::literal_int(1, cond_value);      // cond
    let n2 = Node::literal_int(2, 42);              // then_val
    let n3 = Node::literal_int(3, 99);              // else_val
    let n4 = Node::conditional(4, 1, 2, 3);         // if n1 != 0 then n2 else n3
    let n5 = Node::perform_io_write_line(5, 4);     // print(result)

    let asg = ASG {
        nodes: vec![n1, n2, n3, n4, n5],
        entry: 5,
    };

    save_synapse_file("conditional.synapse", &asg)?;
    println!("Файл conditional.synapse успешно создан!");
    Ok(())
}
