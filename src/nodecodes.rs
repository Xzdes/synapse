use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    LiteralInt, LiteralFloat, LiteralBool, LiteralString, LiteralUnit,
    BinaryOperation, UnaryOperation, Conditional, RecordFieldAccess, Dereference,
    VariableDefinition, VariableReference, Lambda, Application,
    TypeInt, TypeFloat, TypeBool, TypeString, TypeUnit, TypeFunction,
    TypeVariable, ForAll, TypeRecord, FieldDefinition, TypeADT, ADTVariant,
    TypeLinear, TypeSharedRef, TypeMutableRef, TypeLifetime, TypeResult,
    TypeErrorUnion, TypeTrait, TraitMethodDecl, ForeignTypeDecl,
    EffectIO, EffectConsole, EffectFSRead, EffectFSWrite, EffectNetwork,
    EffectState, EffectRandom, EffectExcep, EffectNonTerm, EffectPure,
    DataRecordInit, DataADTInit, DataOk, DataErr,
    PerformEffect, MatchResult, MatchADT, MacroDefinition, MacroInvocation,
    ModuleRoot, ImportDeclaration, ExportDeclaration, ImportAlias,
    ForeignFunctionDecl, ForeignBlock,
    Proof, Specification, Assume, Assert,
    TestCase, TestSuite, Assertion, PropertyDefinition, InputGenerator,
    MatchCase, ImplMethodDef, TraitImpl, Concurrency,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    DataInput, ControlFlowNext, Condition, ThenBranch, ElseBranch,
    ScopeLink, LambdaParameter, LambdaBody, DefinitionLink, BindsVariable,
    ApplicationFunction, ApplicationArgument,
    TypeAnnotation, FunctionParamType, FunctionReturnType,
    TypeVarBinding, TypeBody, Constraint, LinearInnerType,
    RefInnerType, RefLifetime, LifetimeBound, ResultOkType, ResultErrType,
    RecordField, FieldName, FieldType, FieldValue, FieldTarget,
    HasVariant, VariantName, VariantParam, VariantArgValue, VariantTarget,
    ProducesEffect, MatchInput, MatchOkBranch, MatchErrBranch,
    MatchBranch, MatchesVariant, CaseBody,
    ImportsFromModule, ImportsSymbol, ImportsAll, ExportsSymbol, ModuleContains,
    HasFFISignature, UsesABI, LinksToLibrary,
    SpecifiesCode, ProofStepDependsOn, ReliesOnAssumption,
    TestsFunction, ProvidesInput, MakesAssertion, ChecksProperty, InputForProperty,
    MacroBody, MacroInputAST, InvokesMacro,
    ImplementsTrait, ForType, ProvidesImpl, ImplementsMethod,
    RootExpression,
}