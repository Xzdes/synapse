# Synapse

![Rust](https://img.shields.io/badge/Rust-2021-blue)
![Experimental](https://img.shields.io/badge/status-experimental-orange)

**Synapse** — экспериментальный формальный язык программирования и платформа для AI-экспериментов, построенная вокруг *графов абстрактного синтаксиса* (ASG).
Всё, что вы делаете — это манипуляция и выполнение графов!

---

## ✨ Основные возможности

* **Графы вместо синтаксических деревьев:** любая программа — это граф (узлы и ребра), что удобно для анализа и генерации.
* **Бинарный формат SYN1:** кросс-платформенная сериализация/десериализация графов.
* **Интерпретатор на Rust:** CLI-интерпретатор и генераторы графов.
* **Генераторы:** готовые скрипты для арифметики, условий, вывода и т.д.
* **Фабрика узлов:** быстрый и безопасный способ собирать графы через функции-конструкторы.
* **JSON ⇄ SYN1:** конвертеры для AI/ML, визуализации и интеграции с Python.
* **Легко расширять:** новые типы узлов и ребер добавляются одной строкой.
* **Документация и простая структура:** легко вникнуть и начать.

---

## 📚 Структура проекта

```
synapse/
│
├── src/
│   ├── asg.rs             # Основные структуры ASG (Node, Edge, ASG)
│   ├── nodecodes.rs       # Перечисления типов узлов и ребер (NodeType, EdgeType)
│   ├── types.rs           # Типовая система и ошибки (SynType, SynError)
│   ├── node_factories.rs  # Фабрика функций для генераторов узлов
│   ├── syn1.rs            # Загрузка .synapse (SYN1)
│   ├── syn1_writer.rs     # Сохранение .synapse (SYN1)
│   ├── interpreter.rs     # Исполнение графа
│   ├── lib.rs             # Точка входа для библиотеки
│   ├── main.rs            # Пример CLI-интерпретатора
│   └── tools/             # Генераторы и конвертеры:
│       ├── generate_add_print.rs
│       ├── generate_literal_add.rs
│       ├── generate_float_sub.rs
│       ├── generate_conditional.rs
│       ├── generate_gt_if.rs
│       ├── generate_empty_synapse.rs
│       └── convert_synapse_json.rs   # JSON <-> SYN1
│
├── Cargo.toml
├── LICENSE
├── README.md
└── *.synapse / *.json         # Сгенерированные графы и их дампы

```

---

## 🚀 Быстрый старт

### 1. **Установите Rust**

[Официальная инструкция](https://www.rust-lang.org/tools/install)
Рекомендуется Rust 1.70+

```sh
rustup toolchain install stable
```

---

### 2. **Соберите проект**

```sh
git clone https://github.com/Xzdes/synapse.git
cd synapse
cargo build --release
```

---

### 3. **Сгенерируйте пример графа и запустите его**

```sh
cargo run --bin generate_add_print
cargo run --bin synapse add_print.synapse
```

В терминале увидите пошаговое исполнение узлов!

---

### 4. **Конвертируйте граф в JSON (и обратно)**

Экспорт в JSON:

```sh
cargo run --bin convert_synapse_json -- --to-json add_print.synapse add_print.json
```

Импорт из JSON в бинарный формат:

```sh
cargo run --bin convert_synapse_json -- --from-json add_print.json add_print_copy.synapse
```

---

## 🏗 Как писать свои генераторы?

* **Импортируйте фабрику:**

  ```rust
  use synapse::node_factories;
  ```
* **Создавайте узлы так:**

  ```rust
  let n1 = node_factories::literal_int(1, 42);
  let n2 = node_factories::literal_int(2, 100);
  let n3 = node_factories::binary_add(3, 1, 2);
  ```
* **Соберите в граф и сохраните:**

  ```rust
  use synapse::asg::{ASG, Edge};
  use synapse::syn1_writer::save_syn1;
  let asg = ASG { nodes: vec![n1, n2, n3], edges: vec![] };
  save_syn1(&asg, "example.synapse").unwrap();
  ```

---

## 🛠️ Roadmap & планы

**Реализовано:**

* Базовые структуры и типы графов (ASG)
* Бинарный формат SYN1
* Интерпретатор арифметики, условий, вывода
* Все генераторы на новых фабриках
* Конвертер JSON <-> SYN1

**Ближайшее развитие:**

* Расширенная система типов (статическая и динамическая типизация)
* Новые типы узлов (функции, переменные, эффекты)
* Error handling и pattern matching
* FFI (вызов внешнего кода)
* Экспериментальные AI-интеграции

---

## 🤔 Для кого этот проект?

* **Для исследователей:** эксперименты с графами, формальными методами, AI-компиляцией
* **Для разработчиков языков:** создание и тестирование новых узлов на лету
* **Для ML/AI:** прямое создание/модификация/запуск графов программ
* **Для студентов:** отличный старт для изучения языков, графов, Rust

---

## 🤝 Вклад и развитие

* Любые pull requests, фичи, багрепорты — приветствуются!
* Поддерживаем чистоту структуры и явный стиль.
* Используйте `cargo fmt` для форматирования.

---

## 📜 Лицензия

MIT. См. [LICENSE](LICENSE).

---

## 📣 Автор

**Xzdes**
С поддержкой и ревью от сообщества Synapse.

---

> *Проект развивается быстро и открыт для экспериментов. Присоединяйтесь!*