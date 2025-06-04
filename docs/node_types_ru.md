# NodeType — Типы узлов в Synapse

В Synapse каждый узел графа (ASG) имеет тип `NodeType`, который определяет его семантику.  
Ниже приведён список всех типов узлов с кратким описанием.

---

## 📦 Литералы

- **LiteralInt** — целое число.
- **LiteralFloat** — число с плавающей запятой.
- **LiteralBool** — булев тип (true/false).
- **LiteralString** — строка.
- **LiteralUnit** — unit (аналог void).

---

## 🔧 Операции

- **BinaryOperation** — бинарная операция (например, +, -, *, /).
- **UnaryOperation** — унарная операция (например, -a).
- **Conditional** — условная конструкция (if-else).
- **RecordFieldAccess** — доступ к полю записи.
- **Dereference** — разыменование указателя.

---

## 📝 Переменные и функции

- **VariableDefinition** — определение переменной.
- **VariableReference** — ссылка на переменную.
- **Lambda** — лямбда-выражение.
- **Application** — вызов функции.

---

## 📚 Типы

- **TypeInt** — тип Int.
- **TypeFloat** — тип Float.
- **TypeBool** — тип Bool.
- **TypeString** — тип String.
- **TypeUnit** — тип Unit.
- **TypeFunction** — тип функции.
- **TypeVariable** — типовая переменная.
- **ForAll** — обобщённый тип (forall).
- **TypeRecord** — тип записи (Record).
- **FieldDefinition** — определение поля записи.
- **TypeADT** — Algebraic Data Type.
- **ADTVariant** — вариант ADT.
- **TypeLinear** — линейный тип.
- **TypeSharedRef** — shared reference (&T).
- **TypeMutableRef** — mutable reference (&mut T).
- **TypeLifetime** — lifetime.
- **TypeResult** — тип Result.
- **TypeErrorUnion** — тип ErrorUnion.
- **TypeTrait** — тип Trait.
- **TraitMethodDecl** — декларация метода трейта.
- **ForeignTypeDecl** — внешний тип (FFI).

---

## ⚡️ Эффекты

- **EffectIO** — эффект IO.
- **EffectConsole** — вывод в консоль.
- **EffectFSRead** — чтение файла.
- **EffectFSWrite** — запись файла.
- **EffectNetwork** — сетевые взаимодействия.
- **EffectState** — управление состоянием.
- **EffectRandom** — генерация случайных чисел.
- **EffectExcep** — исключение.
- **EffectNonTerm** — недетерминированность.
- **EffectPure** — чистый эффект.

---

## 🏗️ Конструкторы данных

- **DataRecordInit** — инициализация записи.
- **DataADTInit** — инициализация ADT.
- **DataOk** — конструктор Ok.
- **DataErr** — конструктор Err.

---

## 🔌 Макросы и FFI

- **MacroDefinition** — определение макроса.
- **MacroInvocation** — вызов макроса.
- **ForeignFunctionDecl** — декларация внешней функции.
- **ForeignBlock** — блок внешнего кода.

---

## ✅ Доказательства и тестирование

- **Proof** — proof-узел.
- **Specification** — спецификация.
- **Assume** — предположение.
- **Assert** — утверждение.
- **TestCase** — тестовый кейс.
- **TestSuite** — набор тестов.
- **Assertion** — утверждение.
- **PropertyDefinition** — определение свойства.
- **InputGenerator** — генератор входных данных.

---

## 🧩 Модули и трейты

- **ModuleRoot** — корень модуля.
- **ImportDeclaration** — декларация импорта.
- **ExportDeclaration** — декларация экспорта.
- **ImportAlias** — алиас импорта.
- **ImplMethodDef** — определение метода реализации.
- **TraitImpl** — реализация трейта.

---

## 🚀 Другие

- **MatchResult** — результат сопоставления с образцом.
- **MatchADT** — сопоставление ADT.
- **MatchCase** — кейс сопоставления.
- **Concurrency** — многопоточность.

---

🎯 Теперь вы знаете все типы узлов, которые можно использовать в ASG в Synapse.
```

---

✅ В этом файле:

* Подробно расписаны все типы узлов Synapse.
* Для новичков добавлены категории с понятными описаниями.