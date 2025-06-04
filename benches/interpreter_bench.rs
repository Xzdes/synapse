//! Benchmark for interpreter execution (stub).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use synapse::asg::{ASG, NodeID};
use synapse::node_factories::literal_int;
use synapse::interpreter::InterpreterContext;

fn benchmark_interpreter(c: &mut Criterion) {
    c.bench_function("interpreter execution", |b| {
        b.iter(|| {
            let mut asg = ASG::new();
            for id in 1..1000 {
                asg.add_node(literal_int(id, id as i64));
            }

            let interpreter = InterpreterContext;
            interpreter.execute(&asg).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_interpreter);
criterion_main!(benches);
