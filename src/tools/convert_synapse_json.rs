// src/tools/convert_synapse_json.rs

//! Конвертер: SYN1 <-> JSON для ASG-графов Synapse.
//! Использование:
//!   --to-json   input.synapse  output.json
//!   --from-json input.json     output.synapse

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use synapse::asg::ASG;
use synapse::syn1::load_syn1;
use synapse::syn1_writer::save_syn1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage:");
        eprintln!("  --to-json   input.synapse  output.json");
        eprintln!("  --from-json input.json     output.synapse");
        std::process::exit(1);
    }

    let mode = &args[1];
    let input = &args[2];
    let output = &args[3];

    match mode.as_str() {
        "--to-json" => {
            // 1. Читаем .synapse
            let asg = load_syn1(input).expect("Failed to load SYN1 file");

            // 2. Сохраняем .json
            let file = File::create(output).expect("Failed to create JSON file");
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &asg).expect("Failed to write JSON");
            println!("Граф успешно экспортирован в JSON: {output}");
        }
        "--from-json" => {
            // 1. Читаем .json
            let file = File::open(input).expect("Failed to open JSON file");
            let reader = BufReader::new(file);
            let asg: ASG = serde_json::from_reader(reader).expect("Failed to parse JSON");

            // 2. Сохраняем .synapse
            save_syn1(&asg, output).expect("Failed to save SYN1 file");
            println!("Граф успешно импортирован из JSON: {output}");
        }
        _ => {
            eprintln!("Неизвестный режим! Используй --to-json или --from-json");
            std::process::exit(1);
        }
    }
}
