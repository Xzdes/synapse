//! Пример программы на Synapse для демонстрации работы с файлами (заглушка).
//!
//! Здесь мы показываем, как можно использовать ASG для имитации чтения и записи файлов.

use synapse::asg::{ASG, NodeID};
use synapse::node_factories::{literal_string, perform_effect};
use synapse::nodecodes::{NodeType, EdgeType};
use synapse::interpreter::InterpreterContext;

fn main() {
    let mut asg = ASG::new();

    // Создаём узел для имени файла
    let id_file_name: NodeID = 1;
    let file_name_node = literal_string(id_file_name, "example.txt");

    // Создаём узел для эффекта записи в файл
    let id_write_effect: NodeID = 2;
    let mut write_effect_node = perform_effect(id_write_effect, NodeType::EffectFSWrite);

    // Добавляем связь между именем файла и эффектом записи
    write_effect_node.edges.push(synapse::asg::Edge::new(
        EdgeType::DataInput,
        id_file_name,
        None,
    ));

    // Добавляем узлы в ASG
    asg.add_node(file_name_node);
    asg.add_node(write_effect_node);

    // Запускаем интерпретатор
    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}
