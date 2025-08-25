//! Дифференцируемый тензор и логика autograd.
//! Основа этого модуля взята из проекта Rusty Gradients.

// --- ИСПРАВЛЕНИЕ: Убираем неиспользуемый `IxDyn` ---
use ndarray::ArrayD;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// --- Контекст для обратного распространения ---

/// Контекст, который создается в результате операции.
/// Он содержит "указатели" на входные тензоры и функцию,
/// которая знает, как распространить градиент на эти входы.
pub struct BackwardContext {
    pub inputs: Vec<DifferentiableTensor>,
    pub backward_fn: Box<dyn Fn(&ArrayD<f32>)>,
}

impl fmt::Debug for BackwardContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackwardContext")
            .field("num_inputs", &self.inputs.len())
            .finish()
    }
}

// --- Структура Дифференцируемого Тензора ---

/// Тензор, поддерживающий автоматическое дифференцирование.
#[derive(Clone)]
pub struct DifferentiableTensor {
    pub data: Rc<RefCell<ArrayD<f32>>>,
    pub grad: Option<Rc<RefCell<ArrayD<f32>>>>,
    pub ctx: Option<Rc<BackwardContext>>,
}

// --- Реализации стандартных трейтов ---

impl Hash for DifferentiableTensor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.data).hash(state);
    }
}

impl PartialEq for DifferentiableTensor {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for DifferentiableTensor {}

impl fmt::Debug for DifferentiableTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DifferentiableTensor")
            .field("data_shape", &self.data.borrow().shape())
            .field("requires_grad", &self.grad.is_some())
            .finish()
    }
}

// --- Основная логика ---

impl DifferentiableTensor {
    /// Создает новый дифференцируемый тензор.
    pub fn new(data: ArrayD<f32>, requires_grad: bool) -> Self {
        let grad = if requires_grad {
            Some(Rc::new(RefCell::new(ArrayD::zeros(data.shape()))))
        } else {
            None
        };
        Self {
            data: Rc::new(RefCell::new(data)),
            grad,
            ctx: None,
        }
    }

    /// Запускает обратное распространение ошибки, начиная с этого тензора.
    pub fn backward(&self) {
        // --- Логика autograd, перенесенная сюда для простоты ---
        let mut sorted_graph = Vec::new();
        let mut visited = HashSet::new();
        build_graph(self, &mut visited, &mut sorted_graph);

        if let Some(grad) = &self.grad {
            grad.borrow_mut().fill(1.0);
        } else {
            // В настоящей библиотеке здесь должна быть ошибка Result, а не паника.
            panic!("backward() called on a tensor that does not require gradients");
        }

        for t in sorted_graph.iter().rev() {
            if let Some(ctx) = &t.ctx {
                let upstream_grad = t.grad.as_ref().unwrap().borrow();
                (ctx.backward_fn)(&upstream_grad);
            }
        }
    }
}

/// Вспомогательная функция для топологической сортировки графа.
fn build_graph(
    tensor: &DifferentiableTensor,
    visited: &mut HashSet<DifferentiableTensor>,
    sorted: &mut Vec<DifferentiableTensor>,
) {
    if visited.contains(tensor) {
        return;
    }
    visited.insert(tensor.clone());

    if let Some(ctx) = &tensor.ctx {
        for input_tensor in &ctx.inputs {
            build_graph(input_tensor, visited, sorted);
        }
    }
    sorted.push(tensor.clone());
}