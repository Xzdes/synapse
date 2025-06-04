# Synapse

**Synapse** is an AI-friendly programming language and toolkit based on an Abstract Syntax Graph (ASG).  
It is designed for **maximum static analyzability**, **formal proofs**, and easy **AI integration**.

---

## ✅ What’s Done

- **ASG Implementation**: Nodes, edges, and serialization/deserialization (SYN1 format).
- **Interpreter**: Executes simple ASG-based programs.
- **Basic Node Factories**: For creating literals, operations, and effects.
- **Proof System (stub)**: Basic DSL for assertions and specifications.
- **Modular System (stub)**: Import/export declarations and basic module structure.
- **Compiler Backends (stubs)**:
  - LLVM IR
  - WebAssembly
  - C
  - JavaScript
- **Benchmarks**:
  - Serialization/Deserialization
  - Interpreter execution
- **Examples**:
  - Fibonacci (basic arithmetic)
  - HTTP Server (effect simulation)
  - File Processing (effect simulation)
- **Basic Logging**: `env_logger` integrated.
- **CI Setup**: Ready for tests and benchmarks.

---

## 🚧 What’s Next

- **Type Checker**: Type inference and type validation.
- **Proof System**:
  - Connect to external SMT solvers (e.g., Z3).
  - Define proof DSL.
- **FFI System**:
  - Automatic binding generation.
  - Safe wrappers.
- **Compiler Backends**:
  - Implement real LLVM/WASM/C code generation.
  - Optimize generated code.
- **Async Concurrency**:
  - Implement async/await, channels, etc.
- **IDE Integration**:
  - Graph visualizer (Graphviz).
  - VSCode/IntelliJ plugin.
- **Documentation**:
  - More examples.
  - Advanced tutorials.
- **Testing**:
  - Expand unit tests.
  - Add property-based tests.
- **Performance**:
  - Optimize ASG processing and interpreter.
  - Benchmark real-world programs.

---

## 👶 Getting Started

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/synapse.git
   cd synapse
````

2. **Build**:

   ```bash
   cargo build
   ```
3. **Run Tests**:

   ```bash
   cargo test
   ```
4. **Run Benchmarks**:

   ```bash
   cargo bench
   ```
5. **Try Examples**:

   ```bash
   cargo run --example fibonacci
   ```

---

## 🛠️ Contributing

Contributions welcome!
Fork the repository, make your changes, and submit a pull request.

---

## 📜 License

MIT License — see [LICENSE](LICENSE).