# Synapse

![Rust](https://img.shields.io/badge/Rust-2021-blue)
![Experimental](https://img.shields.io/badge/status-experimental-orange)

**Synapse** is an experimental, formal programming language and platform for AI and code analysis, built around *Abstract Syntax Graphs* (ASG).
In Synapse, your programs *are* graphs — you build, transform, and run them directly!

---

## ✨ Features

* **ASG-first:** Every program is a graph (nodes and edges), not a syntax tree. This enables new code analysis and generation techniques.
* **SYN1 binary format:** Cross-platform, compact serialization for graphs.
* **Rust-based interpreter:** CLI interpreter and code generators, written in Rust.
* **Generators:** Ready-made scripts for arithmetic, conditions, print, and more.
* **Node factory:** Type-safe, easy-to-use functions for building graphs.
* **JSON ⇄ SYN1:** Converters for integration with AI/ML or visualization tools (e.g., Python, Jupyter).
* **Easily extensible:** Add new node/edge types with one line.
* **Readable, documented code:** Clean structure for easy onboarding and hacking.

---

## 📚 Project Structure

```
synapse/
│
├── src/
│   ├── asg.rs             # Core ASG structures (Node, Edge, ASG)
│   ├── nodecodes.rs       # NodeType and EdgeType enums (graph vocabulary)
│   ├── types.rs           # Type system and errors (SynType, SynError)
│   ├── node_factories.rs  # Utility functions for node construction
│   ├── syn1.rs            # Binary .synapse (SYN1) loader
│   ├── syn1_writer.rs     # Binary .synapse (SYN1) writer
│   ├── interpreter.rs     # ASG execution engine
│   ├── lib.rs             # Library entry point
│   ├── main.rs            # Example CLI interpreter
│   └── tools/             # CLI generators and converters:
│       ├── generate_add_print.rs
│       ├── generate_literal_add.rs
│       ├── generate_float_sub.rs
│       ├── generate_conditional.rs
│       ├── generate_gt_if.rs
│       ├── generate_empty_synapse.rs
│       └── convert_synapse_json.rs   # JSON <-> SYN1 converter
│
├── Cargo.toml
├── LICENSE
├── README.md
└── *.synapse / *.json         # Generated graph files and JSON dumps

```

---

## 🚀 Quick Start

### 1. **Install Rust**

Follow the [official guide](https://www.rust-lang.org/tools/install).
Recommended: Rust 1.70+

```sh
rustup toolchain install stable
```

---

### 2. **Build the project**

```sh
git clone https://github.com/Xzdes/synapse.git
cd synapse
cargo build --release
```

---

### 3. **Generate and run an example graph**

```sh
cargo run --bin generate_add_print
cargo run --bin synapse add_print.synapse
```

You’ll see step-by-step node execution in your terminal!

---

### 4. **Convert between SYN1 and JSON**

Export to JSON:

```sh
cargo run --bin convert_synapse_json -- --to-json add_print.synapse add_print.json
```

Import from JSON back to binary:

```sh
cargo run --bin convert_synapse_json -- --from-json add_print.json add_print_copy.synapse
```

---

## 🏗 How to write your own generators?

* **Import the node factory:**

  ```rust
  use synapse::node_factories;
  ```
* **Create nodes like this:**

  ```rust
  let n1 = node_factories::literal_int(1, 42);
  let n2 = node_factories::literal_int(2, 100);
  let n3 = node_factories::binary_add(3, 1, 2);
  ```
* **Assemble into a graph and save:**

  ```rust
  use synapse::asg::{ASG, Edge};
  use synapse::syn1_writer::save_syn1;
  let asg = ASG { nodes: vec![n1, n2, n3], edges: vec![] };
  save_syn1(&asg, "example.synapse").unwrap();
  ```

---

## 🛠️ Roadmap & Plans

**Already done:**

* Core ASG structures and enums
* Binary SYN1 format (compact and cross-platform)
* Interpreter for arithmetic, conditions, and print
* All generators rewritten using the new node factory
* JSON <-> SYN1 converter

**Up next:**

* Advanced type system (static/dynamic typing)
* More node/edge types (functions, variables, effect nodes)
* Error handling and pattern matching
* FFI (call external code)
* Experimental AI integrations

---

## 🤔 Who is Synapse for?

* **Researchers:** Experiment with graph-based program analysis, formal methods, or AI-driven compilation.
* **Language designers:** Prototype new node types and see them executed immediately.
* **ML/AI enthusiasts:** Directly generate, mutate, and execute program graphs.
* **Students:** Learn about compilers, graph algorithms, and Rust by example.

---

## 🤝 Contributing

* Pull requests, feature ideas, and bug reports are welcome!
* Please keep modules explicit and code style clean.
* Run `cargo fmt` before submitting PRs.

---

## 📜 License

MIT. See [LICENSE](LICENSE).

---

## 📣 Author

**Xzdes**
With help, code review, and feedback from the Synapse community.

---

> *Synapse is experimental and evolving quickly — join the journey and experiment with us!*