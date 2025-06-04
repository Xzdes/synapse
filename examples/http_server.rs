//! Пример программы на Synapse для демонстрации работы с сетью (заглушка).
//!
//! Здесь мы показываем, как можно использовать ASG для имитации сетевого сервера.

use synapse::asg::{ASG, NodeID};
use synapse::node_factories::{literal_string, perform_effect};
use synapse::nodecodes::{NodeType, EdgeType};
use synapse::interpreter::InterpreterContext;

fn main() {
    let mut asg = ASG::new();

    // Создаём узел, который будет представлять "сервер" (заглушка)
    let id_server: NodeID = 1;
    let server_node = literal_string(id_server, "HTTP Server running on port 8080");

    // Создаём узел для эффекта вывода в консоль
    let id_effect: NodeID = 2;
    let mut effect_node = perform_effect(id_effect, NodeType::EffectConsole);

    // Добавляем связь между сервером и эффектом
    effect_node.edges.push(synapse::asg::Edge::new(
        EdgeType::DataInput,
        id_server,
        None,
    ));

    // Добавляем узлы в ASG
    asg.add_node(server_node);
    asg.add_node(effect_node);

    // Запускаем интерпретатор
    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}
