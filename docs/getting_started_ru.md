# Быстрый старт с Synapse

Добро пожаловать в Synapse! Этот краткий гайд поможет вам начать использовать Synapse в вашем Rust-проекте.

---

## 🛠️ Предварительные требования

- Установлен [Rust](https://www.rust-lang.org/).
- Знание базовых понятий Rust и работы с модулями.

---

## 📦 Установка

Клонируйте репозиторий Synapse:

```bash
git clone https://github.com/yourusername/synapse.git
cd synapse
````

Соберите проект:

```bash
cargo build
```

Запустите тесты для проверки работоспособности:

```bash
cargo test
```

---

## 🚀 Использование Synapse в своём проекте

Чтобы подключить Synapse как зависимость, добавьте его в `Cargo.toml`:

```toml
[dependencies]
synapse = { path = "../synapse" }
```

Теперь можно создавать программы на основе ASG.

Пример:

```rust
use synapse::asg::ASG;
use synapse::node_factories::{literal_int, binary_operation};
use synapse::interpreter::InterpreterContext;

fn main() {
    let mut asg = ASG::new();

    let id1 = 1;
    let id2 = 2;
    let id3 = 3;

    let node1 = literal_int(id1, 5);
    let node2 = literal_int(id2, 8);
    let node3 = binary_operation(id3, "+");

    asg.add_node(node1);
    asg.add_node(node2);
    asg.add_node(node3);

    let interpreter = InterpreterContext;
    interpreter.execute(&asg).unwrap();
}
```

---

## 📚 Подробнее

* [Типы узлов](node_types.md)
* [Типы рёбер](edge_types.md)

---

## 🤝 Контрибуции

Будем рады вашему вкладу! Форкните репозиторий и создайте Pull Request.

---

## 📜 Лицензия

MIT License — см. [LICENSE](../LICENSE).