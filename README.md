## 📄 **README.md**

```markdown
# Synapse

![Rust](https://img.shields.io/badge/Rust-2021-blue)
![Experimental](https://img.shields.io/badge/status-experimental-orange)

**Synapse** is an experimental, AI-oriented, formal programming language and platform.  
It is built around *Abstract Syntax Graphs* (ASG) as the core representation for code —  
enabling optimal code analysis, transformation, verification and generation by both humans and machines.

---

## ✨ Features

- **ASG-first**: Programs are graphs, not just syntax trees.
- **SYN1 Binary Format**: Compact, cross-platform serialization for ASG.
- **Rust Implementation**: Core interpreter, tools, and generators.
- **JSON ⇄ SYN1 Converter**: Export/import graphs for AI/ML and visualization.
- **Test Generators**: Example scripts for arithmetic, conditions, printing, etc.
- **Extensible**: New node/edge types can be added easily.
- **CLI Interpreter**: Run and debug any `.synapse` file.

---

## 📚 Project Structure

```

synapse/
│
├── src/
│   ├── asg.rs               # ASG structures (Node, Edge, ASG)
│   ├── nodecodes.rs         # NodeType/EdgeType enums (graph vocabulary)
│   ├── syn1.rs              # Binary file loader
│   ├── syn1\_writer.rs       # Binary file writer
│   ├── interpreter.rs       # ASG execution engine
│   ├── types.rs             # (Stub) Type system
│   ├── lib.rs               # Library entry point
│   ├── main.rs              # CLI interpreter
│   └── tools/               # CLI tools/generators:
│       ├── generate\_add\_print.rs
│       ├── generate\_conditional.rs
│       ├── generate\_empty\_synapse.rs
│       ├── generate\_gt\_if.rs
│       ├── generate\_literal\_add.rs
│       ├── generate\_float\_sub.rs
│       └── convert\_synapse\_json.rs   # JSON <-> SYN1 converter
│
├── Cargo.toml
├── LICENSE
├── README.md
└── \*.synapse / \*.json       # Generated files

````

---

## 🚀 Getting Started

### 1. **Prerequisites**

- Rust 1.70+ (`rustup toolchain install stable`)
- [Cargo](https://www.rust-lang.org/tools/install)

### 2. **Build everything**

```sh
git clone https://github.com/Xzdes/synapse.git
cd synapse
cargo build --release
````

### 3. **Try the CLI Interpreter**

Generate an example graph and run it:

```sh
cargo run --bin generate_add_print
cargo run --bin synapse add_print.synapse
```

You’ll see detailed node-by-node output!

### 4. **Convert between SYN1 and JSON**

Export `.synapse` to `.json`:

```sh
cargo run --bin convert_synapse_json -- --to-json add_print.synapse add_print.json
```

Import JSON back to SYN1:

```sh
cargo run --bin convert_synapse_json -- --from-json add_print.json add_print_copy.synapse
```

---

## 🛠️ Roadmap & Plans

### **Done**

* [x] Core ASG structs and node/edge enums
* [x] SYN1 binary file format
* [x] Interpreter with arithmetic, branching, and I/O
* [x] Tool generators (arithmetic, conditionals, float/int support)
* [x] JSON ⇄ SYN1 converter

### **Upcoming**

* [ ] Type system (static/dynamic typing, polymorphism)
* [ ] More node/edge types (functions, variables, effect nodes)
* [ ] Error handling (Result, pattern matching)
* [ ] FFI and external calls
* [ ] Visualization tools (e.g. Graphviz export)
* [ ] AI-assisted code generation (experiment, auto-synthesis)
* [ ] Formal verification (assume/assert nodes, proof system)

---

## 🤔 Why Synapse?

* **For researchers**: Experiment with graph-based representations, code analysis, or AI-driven compilation.
* **For language designers**: Prototype new constructs as node types and see them executed instantly.
* **For AI/ML**: Generate or mutate graphs directly, then execute or verify them.
* **For students/enthusiasts**: See “how the sausage is made” — graphs, not just trees!

---

## 🤝 Contributing

* Issues, feature requests and PRs are welcome!
* File layout and Rust style: please keep everything explicit, don’t abbreviate code in main modules (for accessibility).
* Use `cargo fmt` before PR.

---

## 📜 License

MIT. See [LICENSE](LICENSE) for details.

---

## 📣 Author

**Xzdes**

---

*This project is experimental, but growing fast — stay tuned for updates!*

