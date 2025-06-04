//! Модуль `nodecodes`
//!
//! Содержит перечисления:
//! - `NodeType`: все узлы, включая литералы, операции, типы, эффекты, proof, FFI и др.
//! - `EdgeType`: все рёбра, включая Data/Control Flow, Scope/Binding, Effects, Verification и др.
//!
//! Все публичные перечисления сериализуемы через serde.

use serde::{Deserialize, Serialize};

/// Перечисление всех типов узлов в ASG.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    // 4.1. Literals
    LiteralInt,
    LiteralFloat,
    LiteralBool,
    LiteralString,
    LiteralUnit,

    // 4.2. Operations
    BinaryOperation, // payload: operator code
    UnaryOperation,  // payload: operator code
    Conditional,
    RecordFieldAccess,
    Dereference,

    // 4.3. Variables and Functions
    VariableDefinition,
    VariableReference,
    Lambda,
    Application,

    // 4.4. Types
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeString,
    TypeUnit,
    TypeFunction,
    TypeVariable,
    ForAll,
    TypeRecord,
    FieldDefinition,
    TypeADT,
    ADTVariant,
    TypeLinear,
    TypeSharedRef,
    TypeMutableRef,
    TypeLifetime,
    TypeResult,
    TypeErrorUnion,
    TypeTrait,
    TraitMethodDecl,
    ForeignTypeDecl,

    // 4.5. Effects
    EffectIO,
    EffectConsole,
    EffectFSRead,
    EffectFSWrite,
    EffectNetwork,
    EffectState,
    EffectRandom,
    EffectExcep,
    EffectNonTerm,
    EffectPure,

    // 4.6. Data Constructors
    DataRecordInit,
    DataADTInit,
    DataOk,
    DataErr,

    // 4.7. Effect Operations
    PerformEffect,

    // 4.8. Pattern Matching
    MatchResult,
    MatchADT,

    // 4.9. Metaprogramming
    MacroDefinition,
    MacroInvocation,

    // 4.10. Modules
    ModuleRoot,
    ImportDeclaration,
    ExportDeclaration,
    ImportAlias,

    // 4.11. FFI
    ForeignFunctionDecl,
    ForeignBlock,

    // 4.12. Verification
    Proof,
    Specification,
    Assume,
    Assert,

    // 4.13. Testing
    TestCase,
    TestSuite,
    Assertion,
    PropertyDefinition,
    InputGenerator,

    // 4.14. Helpers
    MatchCase,
    ImplMethodDef,
    TraitImpl,

    // 🆕 Добавляем поддержку многопоточности
    Concurrency,
}

/// Перечисление всех типов рёбер в ASG.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    // 5.1. Data/Control Flow
    DataInput,
    ControlFlowNext,
    Condition,
    ThenBranch,
    ElseBranch,

    // 5.2. Scope/Binding
    ScopeLink,
    LambdaParameter,
    LambdaBody,
    DefinitionLink,
    BindsVariable,

    // 5.3. Function Calls
    ApplicationFunction,
    ApplicationArgument,

    // 5.4. Typing
    TypeAnnotation,
    FunctionParamType,
    FunctionReturnType,
    TypeVarBinding,
    TypeBody,
    Constraint,
    LinearInnerType,
    RefInnerType,
    RefLifetime,
    LifetimeBound,
    ResultOkType,
    ResultErrType,

    // 5.5. Data Structures
    RecordField,
    FieldName,
    FieldType,
    FieldValue,
    FieldTarget,
    HasVariant,
    VariantName,
    VariantParam,
    VariantArgValue,
    VariantTarget,

    // 5.6. Effects
    ProducesEffect,

    // 5.7. Pattern Matching
    MatchInput,
    MatchOkBranch,
    MatchErrBranch,
    MatchBranch,
    MatchesVariant,
    CaseBody,

    // 5.8. Modules
    ImportsFromModule,
    ImportsSymbol,
    ImportsAll,
    ExportsSymbol,
    ModuleContains,

    // 5.9. FFI
    HasFFISignature,
    UsesABI,
    LinksToLibrary,

    // 5.10. Verification
    ProvesSpec,
    SpecifiesCode,
    ProofStepDependsOn,
    ReliesOnAssumption,

    // 5.11. Testing
    TestsFunction,
    ProvidesInput,
    MakesAssertion,
    ChecksProperty,
    InputForProperty,

    // 5.12. Metaprogramming
    MacroBody,
    MacroInputAST,
    InvokesMacro,

    // 5.13. Traits/Implementations
    HasMethod,
    ImplementsTrait,
    ForType,
    ProvidesImpl,
    ImplementsMethod,

    // 5.14. Structural
    RootExpression,
}
