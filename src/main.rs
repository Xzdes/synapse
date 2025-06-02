use std::env;
use synapse::asg::ASG;
use synapse::syn1::load_synapse_file;
use synapse::interpreter::execute;

fn main() {
    // Получаем имя файла из аргументов командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: synapse <input.synapse>");
        return;
    }

    let filename = &args[1];
    match load_synapse_file(filename) {
        Ok(asg) => {
            println!("Loaded ASG with {} nodes", asg.nodes.len());
            if let Err(e) = execute(&asg) {
                println!("Ошибка интерпретации: {e}");
            }
        }
        Err(e) => {
            println!("Ошибка загрузки файла: {e}");
        }
    }
}
