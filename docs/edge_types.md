# EdgeType — Synapse Edge Types

In Synapse, each edge in the ASG (Abstract Syntax Graph) has a type called `EdgeType` that defines its semantics.  
Below is a list of all edge types with short descriptions.

---

## 🔗 Data and Control Flow

- **DataInput** — data argument.
- **ControlFlowNext** — next control node.
- **Condition** — branch condition.
- **ThenBranch** — then branch.
- **ElseBranch** — else branch.

---

## 📦 Scope and Binding

- **ScopeLink** — scope linking.
- **LambdaParameter** — lambda parameter.
- **LambdaBody** — lambda body.
- **DefinitionLink** — definition linking.
- **BindsVariable** — variable binding.

---

## 📞 Function Calls

- **ApplicationFunction** — function being called.
- **ApplicationArgument** — function argument.

---

## 📝 Type Annotations

- **TypeAnnotation** — type annotation.
- **FunctionParamType** — function parameter type.
- **FunctionReturnType** — function return type.
- **TypeVarBinding** — type variable binding.
- **TypeBody** — type body.
- **Constraint** — type constraint.
- **LinearInnerType** — linear type inner.
- **RefInnerType** — reference type inner.
- **RefLifetime** — reference lifetime.
- **LifetimeBound** — lifetime bound.
- **ResultOkType** — Ok type of Result.
- **ResultErrType** — Err type of Result.

---

## 🏗️ Data Structures

- **RecordField** — record field.
- **FieldName** — field name.
- **FieldType** — field type.
- **FieldValue** — field value.
- **FieldTarget** — target field.
- **HasVariant** — ADT has variant.
- **VariantName** — variant name.
- **VariantParam** — variant parameter.
- **VariantArgValue** — variant argument value.
- **VariantTarget** — target variant.

---

## ⚡️ Effects

- **ProducesEffect** — effect node.

---

## 🧩 Pattern Matching

- **MatchInput** — input to match.
- **MatchOkBranch** — Ok branch.
- **MatchErrBranch** — Err branch.
- **MatchBranch** — match branch.
- **MatchesVariant** — matches variant.
- **CaseBody** — case body.

---

## 📦 Modules

- **ImportsFromModule** — imports from module.
- **ImportsSymbol** — imports a symbol.
- **ImportsAll** — imports everything.
- **ExportsSymbol** — exports a symbol.
- **ModuleContains** — module contains node.

---

## 🔌 FFI

- **HasFFISignature** — FFI function signature.
- **UsesABI** — ABI usage.
- **LinksToLibrary** — links to external library.

---

## ✅ Proofs and Specifications

- **SpecifiesCode** — links to specification.
- **ProofStepDependsOn** — proof step dependency.
- **ReliesOnAssumption** — depends on an assumption.

---

## 🧪 Testing

- **TestsFunction** — tests a function.
- **ProvidesInput** — provides input.
- **MakesAssertion** — makes an assertion.
- **ChecksProperty** — checks a property.
- **InputForProperty** — input for a property.

---

## 🔧 Macros

- **MacroBody** — macro body.
- **MacroInputAST** — macro input AST.
- **InvokesMacro** — invokes a macro.

---

## 🏗️ Traits and Implementations

- **ImplementsTrait** — implements a trait.
- **ForType** — for a type.
- **ProvidesImpl** — provides implementation.
- **ImplementsMethod** — implements a method.

---

## 🚀 Structural

- **RootExpression** — root expression.

---

🎯 Now you know all the edge types you can use in the Synapse ASG!