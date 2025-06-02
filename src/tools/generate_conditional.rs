use synapse_project::asg::{ASG, Node};
use synapse_project::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    // Пример: if (cond) { then_val } else { else_val } => print(result)
    // Измените значение cond ниже (1 или 0), чтобы менять ветку

    let cond_value: i64 = 1; // попробуй 0 для else-ветки

    // 1. Условие
    let n1 = Node::literal_int(1, cond_value);      // cond
    // 2. then-значение
    let n2 = Node::literal_int(2, 42);     // then_val
    // 3. else-значение
    let n3 = Node::literal_int(3, 99);     // else_val
    // 4. Условие: if cond != 0 then then_val else else_val
    let n4 = Node::conditional(4, 1, 2, 3);
    // 5. Печать результата
    let n5 = Node::perform_io_write_line(5, 4);

    let asg = ASG {
        nodes: vec![n1, n2, n3, n4, n5],
        entry: 5,
    };

    save_synapse_file("conditional.synapse", &asg)?;
    println!("Файл conditional.synapse успешно создан!");
    Ok(())
}
