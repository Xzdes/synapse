# Synapse

**Synapse** is an experimental AI-focused programming language that uses an **Abstract Syntax Graph (ASG)** as its primary representation.  
This repository contains the core library that defines the ASG, node/edge types, serialization (SYN1 format), interpreter, proof system, modules, FFI, effects, and more.

---

## 🧩 What is Synapse?

Synapse is a formal programming language designed with the following goals:
- **Formal rigor and verifiability** — ensures AI-generated code is analyzable and verifiable.
- **ASG as the canonical representation** — unlike traditional text-based languages, Synapse treats programs as graphs of nodes and edges.
- **Static analyzability** — every node and edge is fully typed and documented.
- **Cross-platform execution** — runs on Windows, Linux, macOS, Android, iOS, and WebAssembly (where possible).

---

## ✨ Features

✅ Abstract Syntax Graph (ASG)  
✅ Binary serialization format (SYN1)  
✅ Interpreter (runs ASG programs)  
✅ Proof and specification support (assertions, assumes)  
✅ Effects system (IO, File, Console, etc.)  
✅ Macro system  
✅ Multithreading support  
✅ Modular architecture  
✅ JSON import/export  
✅ Unit tests included

---

## 🛠️ How to build

You'll need [Rust](https://www.rust-lang.org/) installed.

```bash
git clone https://github.com/yourusername/synapse.git
cd synapse
cargo build
````

---

## 🚀 How to run

You can run tests with:

```bash
cargo test
```

If you want to explore how to use Synapse in your own Rust project, you can include it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
synapse = { path = "../synapse" }
```

Then you can use it like this:

```rust
use synapse::asg::ASG;
use synapse::interpreter::InterpreterContext;
use synapse::node_factories::literal_int;

fn main() {
    let mut asg = ASG::new();
    let node = literal_int(1, 42);
    asg.add_node(node);

    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}
```

---

## 📦 What's inside?

* `asg.rs` — defines ASG, nodes, and edges
* `nodecodes.rs` — enums for NodeType and EdgeType
* `syn1.rs` — loader for binary SYN1 format
* `syn1_writer.rs` — writer for binary SYN1 format
* `interpreter.rs` — ASG interpreter
* `node_factories.rs` — helper functions for building nodes
* `effects.rs` — effect system (IO, console, file, etc.)
* `proof.rs` — proof and specification support
* `ffi.rs` — foreign function interface
* `macros.rs` — macro system
* `concurrency.rs` — thread support
* `testing.rs` — unit test integration
* `compiler.rs` — frontend/backend architecture
* `ai_api.rs` — AI interface (JSON serialization)
* `types.rs` — Synapse type system

---

## 🤝 Contributing

Contributions are welcome! Please fork this repo and create a pull request.
If you'd like to discuss features, open an issue or email me.

---

## 📜 License

MIT License — see [LICENSE](LICENSE).