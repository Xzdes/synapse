# NodeType — Synapse Node Types

In Synapse, each node in the ASG (Abstract Syntax Graph) has a type called `NodeType` that defines its semantics.  
Below is a list of all node types with short descriptions.

---

## 📦 Literals

- **LiteralInt** — an integer literal.
- **LiteralFloat** — a floating-point literal.
- **LiteralBool** — a boolean literal (true/false).
- **LiteralString** — a string literal.
- **LiteralUnit** — a unit literal (similar to void).

---

## 🔧 Operations

- **BinaryOperation** — a binary operation (e.g., +, -, *, /).
- **UnaryOperation** — a unary operation (e.g., -a).
- **Conditional** — a conditional expression (if-else).
- **RecordFieldAccess** — access to a record field.
- **Dereference** — pointer dereferencing.

---

## 📝 Variables and Functions

- **VariableDefinition** — a variable definition.
- **VariableReference** — a variable reference.
- **Lambda** — a lambda expression.
- **Application** — a function application.

---

## 📚 Types

- **TypeInt** — the Int type.
- **TypeFloat** — the Float type.
- **TypeBool** — the Bool type.
- **TypeString** — the String type.
- **TypeUnit** — the Unit type.
- **TypeFunction** — a function type.
- **TypeVariable** — a type variable.
- **ForAll** — a polymorphic type (forall).
- **TypeRecord** — a record type.
- **FieldDefinition** — a record field definition.
- **TypeADT** — an Algebraic Data Type.
- **ADTVariant** — a variant of an ADT.
- **TypeLinear** — a linear type.
- **TypeSharedRef** — a shared reference (&T).
- **TypeMutableRef** — a mutable reference (&mut T).
- **TypeLifetime** — a lifetime.
- **TypeResult** — the Result type.
- **TypeErrorUnion** — the ErrorUnion type.
- **TypeTrait** — a Trait type.
- **TraitMethodDecl** — a trait method declaration.
- **ForeignTypeDecl** — a foreign type (FFI).

---

## ⚡️ Effects

- **EffectIO** — IO effect.
- **EffectConsole** — console output.
- **EffectFSRead** — file reading.
- **EffectFSWrite** — file writing.
- **EffectNetwork** — network interactions.
- **EffectState** — state management.
- **EffectRandom** — random number generation.
- **EffectExcep** — exceptions.
- **EffectNonTerm** — non-termination.
- **EffectPure** — pure effect.

---

## 🏗️ Data Constructors

- **DataRecordInit** — record initialization.
- **DataADTInit** — ADT initialization.
- **DataOk** — Ok constructor.
- **DataErr** — Err constructor.

---

## 🔌 Macros and FFI

- **MacroDefinition** — macro definition.
- **MacroInvocation** — macro invocation.
- **ForeignFunctionDecl** — external function declaration.
- **ForeignBlock** — external code block.

---

## ✅ Proofs and Testing

- **Proof** — a proof node.
- **Specification** — a specification.
- **Assume** — an assumption.
- **Assert** — an assertion.
- **TestCase** — a test case.
- **TestSuite** — a test suite.
- **Assertion** — an assertion node.
- **PropertyDefinition** — a property definition.
- **InputGenerator** — an input generator.

---

## 🧩 Modules and Traits

- **ModuleRoot** — module root.
- **ImportDeclaration** — import declaration.
- **ExportDeclaration** — export declaration.
- **ImportAlias** — import alias.
- **ImplMethodDef** — implementation method definition.
- **TraitImpl** — trait implementation.

---

## 🚀 Others

- **MatchResult** — result of pattern matching.
- **MatchADT** — ADT pattern matching.
- **MatchCase** — match case.
- **Concurrency** — concurrency support.

---

🎯 Now you know all the node types you can use in the Synapse ASG!