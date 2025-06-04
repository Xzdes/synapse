# EdgeType — Типы рёбер в Synapse

В Synapse каждое ребро графа (ASG) имеет тип `EdgeType`, который определяет его семантику.  
Ниже приведён список всех типов рёбер с кратким описанием.

---

## 🔗 Поток данных и управление

- **DataInput** — аргумент данных.
- **ControlFlowNext** — следующий управляющий узел.
- **Condition** — условие ветвления.
- **ThenBranch** — ветвь then.
- **ElseBranch** — ветвь else.

---

## 📦 Область видимости и привязка

- **ScopeLink** — связь областей видимости.
- **LambdaParameter** — параметр лямбды.
- **LambdaBody** — тело лямбды.
- **DefinitionLink** — связь определения.
- **BindsVariable** — связывание переменной.

---

## 📞 Вызовы функций

- **ApplicationFunction** — вызываемая функция.
- **ApplicationArgument** — аргумент вызова функции.

---

## 📝 Аннотации типов

- **TypeAnnotation** — аннотация типа.
- **FunctionParamType** — тип параметра функции.
- **FunctionReturnType** — тип возвращаемого значения функции.
- **TypeVarBinding** — связывание переменной типа.
- **TypeBody** — тело типа.
- **Constraint** — ограничение типа.
- **LinearInnerType** — внутренний линейный тип.
- **RefInnerType** — внутренний тип ссылки.
- **RefLifetime** — lifetime ссылки.
- **LifetimeBound** — ограничение lifetime.
- **ResultOkType** — тип Ok в Result.
- **ResultErrType** — тип Err в Result.

---

## 🏗️ Структуры данных

- **RecordField** — поле записи.
- **FieldName** — имя поля.
- **FieldType** — тип поля.
- **FieldValue** — значение поля.
- **FieldTarget** — целевое поле.
- **HasVariant** — наличие варианта в ADT.
- **VariantName** — имя варианта.
- **VariantParam** — параметр варианта.
- **VariantArgValue** — аргумент варианта.
- **VariantTarget** — целевой вариант.

---

## ⚡️ Эффекты

- **ProducesEffect** — узел эффекта.

---

## 🧩 Сопоставление с образцом

- **MatchInput** — вход для сопоставления.
- **MatchOkBranch** — ветвь Ok.
- **MatchErrBranch** — ветвь Err.
- **MatchBranch** — ветвь сопоставления.
- **MatchesVariant** — совпадение с вариантом.
- **CaseBody** — тело кейса.

---

## 📦 Модули

- **ImportsFromModule** — импорт из модуля.
- **ImportsSymbol** — импорт символа.
- **ImportsAll** — импорт всего.
- **ExportsSymbol** — экспорт символа.
- **ModuleContains** — модуль содержит узел.

---

## 🔌 FFI

- **HasFFISignature** — сигнатура функции FFI.
- **UsesABI** — использование ABI.
- **LinksToLibrary** — ссылка на внешнюю библиотеку.

---

## ✅ Доказательства и спецификации

- **SpecifiesCode** — связь со спецификацией.
- **ProofStepDependsOn** — зависимость шага доказательства.
- **ReliesOnAssumption** — зависимость от предположения.

---

## 🧪 Тестирование

- **TestsFunction** — тестирование функции.
- **ProvidesInput** — обеспечение входных данных.
- **MakesAssertion** — утверждение.
- **ChecksProperty** — проверка свойства.
- **InputForProperty** — входные данные для свойства.

---

## 🔧 Макросы

- **MacroBody** — тело макроса.
- **MacroInputAST** — AST вход макроса.
- **InvokesMacro** — вызов макроса.

---

## 🏗️ Трейты и реализации

- **ImplementsTrait** — реализация трейта.
- **ForType** — для типа.
- **ProvidesImpl** — предоставляет реализацию.
- **ImplementsMethod** — реализация метода.

---

## 🚀 Структура

- **RootExpression** — корневое выражение.

---

🎯 Теперь вы знаете все типы рёбер, которые можно использовать в ASG в Synapse!