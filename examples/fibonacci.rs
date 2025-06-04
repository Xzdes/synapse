//! Пример программы на Synapse для вычисления чисел Фибоначчи.
//!
//! Демонстрирует создание узлов и рёбер вручную и их исполнение через интерпретатор.

use synapse::asg::{ASG, NodeID};
use synapse::node_factories::{literal_int, binary_operation};
use synapse::interpreter::InterpreterContext;

fn main() {
    let mut asg = ASG::new();

    // Пример простого вычисления: fib(5)
    // Для упрощения создадим узлы для 5 и 8, а потом сложим их.

    let id1: NodeID = 1;
    let id2: NodeID = 2;
    let id3: NodeID = 3;

    let node1 = literal_int(id1, 5);
    let node2 = literal_int(id2, 8);
    // Здесь исправлено: передаём оператор как u8 (1 = '+')
    let node3 = binary_operation(id3, 1);

    // Связываем аргументы с операцией
    let mut node3 = node3;
    node3.edges.push(synapse::asg::Edge::new(
        synapse::nodecodes::EdgeType::ApplicationArgument,
        id1,
        None,
    ));
    node3.edges.push(synapse::asg::Edge::new(
        synapse::nodecodes::EdgeType::ApplicationArgument,
        id2,
        None,
    ));

    // Добавляем узлы в ASG
    asg.add_node(node1);
    asg.add_node(node2);
    asg.add_node(node3);

    // Запускаем интерпретатор
    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}
