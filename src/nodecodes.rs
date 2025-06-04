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
    /// Литерал целого числа.
    LiteralInt,
    /// Литерал числа с плавающей запятой.
    LiteralFloat,
    /// Литерал булевого значения.
    LiteralBool,
    /// Литерал строки.
    LiteralString,
    /// Литерал unit.
    LiteralUnit,
    /// Бинарная операция.
    BinaryOperation,
    /// Унарная операция.
    UnaryOperation,
    /// Условная конструкция (if-else).
    Conditional,
    /// Доступ к полю записи.
    RecordFieldAccess,
    /// Разыменование указателя.
    Dereference,
    /// Определение переменной.
    VariableDefinition,
    /// Ссылка на переменную.
    VariableReference,
    /// Лямбда-выражение.
    Lambda,
    /// Вызов функции.
    Application,
    /// Тип Int.
    TypeInt,
    /// Тип Float.
    TypeFloat,
    /// Тип Bool.
    TypeBool,
    /// Тип String.
    TypeString,
    /// Тип Unit.
    TypeUnit,
    /// Тип функции.
    TypeFunction,
    /// Типовая переменная.
    TypeVariable,
    /// Обобщённый тип (ForAll).
    ForAll,
    /// Тип записи (Record).
    TypeRecord,
    /// Определение поля записи.
    FieldDefinition,
    /// Algebraic Data Type.
    TypeADT,
    /// Вариант ADT.
    ADTVariant,
    /// Линейный тип.
    TypeLinear,
    /// Shared reference (&T).
    TypeSharedRef,
    /// Mutable reference (&mut T).
    TypeMutableRef,
    /// Lifetime.
    TypeLifetime,
    /// Тип Result.
    TypeResult,
    /// Тип ErrorUnion.
    TypeErrorUnion,
    /// Тип Trait.
    TypeTrait,
    /// Декларация метода трейта.
    TraitMethodDecl,
    /// Foreign type (FFI).
    ForeignTypeDecl,
    /// Эффект: IO.
    EffectIO,
    /// Эффект: Console.
    EffectConsole,
    /// Эффект: чтение файлов.
    EffectFSRead,
    /// Эффект: запись файлов.
    EffectFSWrite,
    /// Эффект: Network.
    EffectNetwork,
    /// Эффект: State.
    EffectState,
    /// Эффект: Random.
    EffectRandom,
    /// Эффект: Exception.
    EffectExcep,
    /// Эффект: Non-termination.
    EffectNonTerm,
    /// Эффект: Pure.
    EffectPure,
    /// Инициализация записи.
    DataRecordInit,
    /// Инициализация ADT.
    DataADTInit,
    /// Конструктор DataOk.
    DataOk,
    /// Конструктор DataErr.
    DataErr,
    /// Выполнение эффекта.
    PerformEffect,
    /// Результат сопоставления с образцом.
    MatchResult,
    /// Сопоставление ADT.
    MatchADT,
    /// Определение макроса.
    MacroDefinition,
    /// Вызов макроса.
    MacroInvocation,
    /// Корень модуля.
    ModuleRoot,
    /// Импорт декларации.
    ImportDeclaration,
    /// Экспорт декларации.
    ExportDeclaration,
    /// Алиас для импорта.
    ImportAlias,
    /// Декларация внешней функции (FFI).
    ForeignFunctionDecl,
    /// Блок внешнего кода (FFI).
    ForeignBlock,
    /// Proof-узел.
    Proof,
    /// Specification-узел.
    Specification,
    /// Assume-узел.
    Assume,
    /// Assert-узел.
    Assert,
    /// Тестовый кейс.
    TestCase,
    /// Набор тестов.
    TestSuite,
    /// Assertion-узел.
    Assertion,
    /// Определение свойства.
    PropertyDefinition,
    /// Генератор входных данных.
    InputGenerator,
    /// Узел сопоставления с кейсом.
    MatchCase,
    /// Определение метода реализации.
    ImplMethodDef,
    /// Реализация трейта.
    TraitImpl,
    /// Узел для многопоточности.
    Concurrency,
}

/// Перечисление всех типов рёбер в ASG.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    /// Данные (аргумент).
    DataInput,
    /// Следующий узел управления.
    ControlFlowNext,
    /// Условие ветвления.
    Condition,
    /// Ветвь then.
    ThenBranch,
    /// Ветвь else.
    ElseBranch,
    /// Связь областей видимости.
    ScopeLink,
    /// Параметр лямбды.
    LambdaParameter,
    /// Тело лямбды.
    LambdaBody,
    /// Связь определения.
    DefinitionLink,
    /// Связывает переменные.
    BindsVariable,
    /// Функция вызова.
    ApplicationFunction,
    /// Аргумент вызова.
    ApplicationArgument,
    /// Аннотация типа.
    TypeAnnotation,
    /// Тип параметра функции.
    FunctionParamType,
    /// Тип возвращаемого значения функции.
    FunctionReturnType,
    /// Связывание переменной типа.
    TypeVarBinding,
    /// Тело типа.
    TypeBody,
    /// Ограничение типа.
    Constraint,
    /// Линейный тип (внутри).
    LinearInnerType,
    /// Внутренний тип ссылки.
    RefInnerType,
    /// Lifetime ссылки.
    RefLifetime,
    /// Ограничение lifetime.
    LifetimeBound,
    /// Тип Ok в Result.
    ResultOkType,
    /// Тип Err в Result.
    ResultErrType,
    /// Поле записи.
    RecordField,
    /// Имя поля.
    FieldName,
    /// Тип поля.
    FieldType,
    /// Значение поля.
    FieldValue,
    /// Целевое поле.
    FieldTarget,
    /// Наличие варианта.
    HasVariant,
    /// Имя варианта.
    VariantName,
    /// Параметр варианта.
    VariantParam,
    /// Аргумент варианта.
    VariantArgValue,
    /// Целевой вариант.
    VariantTarget,
    /// Узел эффекта.
    ProducesEffect,
    /// Вход сопоставления.
    MatchInput,
    /// Ветвь Ok.
    MatchOkBranch,
    /// Ветвь Err.
    MatchErrBranch,
    /// Ветвь сопоставления.
    MatchBranch,
    /// Совпадает с вариантом.
    MatchesVariant,
    /// Тело кейса.
    CaseBody,
    /// Импорт из модуля.
    ImportsFromModule,
    /// Импорт символа.
    ImportsSymbol,
    /// Импорт всего.
    ImportsAll,
    /// Экспорт символа.
    ExportsSymbol,
    /// Модуль содержит.
    ModuleContains,
    /// FFI: сигнатура.
    HasFFISignature,
    /// FFI: ABI.
    UsesABI,
    /// FFI: ссылка на библиотеку.
    LinksToLibrary,
    /// Указывает спецификацию.
    SpecifiesCode,
    /// Связывает шаги доказательства.
    ProofStepDependsOn,
    /// Зависит от предположения.
    ReliesOnAssumption,
    /// Тестовые связи: вызывает функцию.
    TestsFunction,
    /// Тестовые связи: обеспечивает входные данные.
    ProvidesInput,
    /// Тестовые связи: выполняет утверждение.
    MakesAssertion,
    /// Тестовые связи: проверяет свойство.
    ChecksProperty,
    /// Тестовые связи: вход для свойства.
    InputForProperty,
    /// Макросы: тело макроса.
    MacroBody,
    /// Макросы: AST вход.
    MacroInputAST,
    /// Макросы: вызывает макрос.
    InvokesMacro,
    /// Методы трейтов: реализует трейт.
    ImplementsTrait,
    /// Методы трейтов: для типа.
    ForType,
    /// Методы трейтов: предоставляет реализацию.
    ProvidesImpl,
    /// Методы трейтов: реализует метод.
    ImplementsMethod,
    /// Корневое выражение.
    RootExpression,
}
