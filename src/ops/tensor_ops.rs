//! Операции над дифференцируемыми тензорами.

use crate::runtime::diff_tensor::{BackwardContext, DifferentiableTensor};
use ndarray::{ArrayD, Axis};
use std::rc::Rc; // Импортируем Rc

// Функция `reduce_grad`, скопированная из `basic_ops.rs`
fn reduce_grad(upstream: &ArrayD<f32>, target_shape: &[usize]) -> ArrayD<f32> {
    let mut current = upstream.clone();
    let upstream_shape = current.shape();
    
    let trim_dims = upstream_shape.len().saturating_sub(target_shape.len());
    for _ in 0..trim_dims {
        current = current.sum_axis(Axis(0));
    }

    let mut axes_to_sum = Vec::new();
    for (i, (&upstream_dim, &target_dim)) in current.shape().iter().zip(target_shape.iter()).enumerate() {
        if upstream_dim != target_dim {
            if target_dim == 1 {
                axes_to_sum.push(i);
            } else {
                 panic!(
                    "Cannot reduce grad from shape {:?} to {:?}, dimension mismatch at axis {}",
                    upstream.shape(), target_shape, i
                );
            }
        }
    }
    
    for axis_idx in axes_to_sum.iter().rev() {
        current = current.sum_axis(Axis(*axis_idx));
    }

    if current.shape() != target_shape {
        current = current
            .into_shape_with_order(target_shape.to_vec())
            .expect("reduce_grad: final reshape failed");
    }

    current
}

/// Выполняет сложение двух дифференцируемых тензоров.
pub fn add(a: &DifferentiableTensor, b: &DifferentiableTensor) -> DifferentiableTensor {
    let lhs_data = a.data.borrow();
    let rhs_data = b.data.borrow();
    let result_data = &*lhs_data + &*rhs_data;
    let requires_grad = a.grad.is_some() || b.grad.is_some();
    let mut result = DifferentiableTensor::new(result_data, requires_grad);

    if requires_grad {
        let lhs_shape = lhs_data.shape().to_vec();
        let rhs_shape = rhs_data.shape().to_vec();
        let lhs_for_closure = a.clone();
        let rhs_for_closure = b.clone();
        let backward_fn = Box::new(move |upstream_grad: &ArrayD<f32>| {
            if let Some(grad_lhs) = &lhs_for_closure.grad {
                let reduced = reduce_grad(upstream_grad, &lhs_shape);
                grad_lhs.borrow_mut().scaled_add(1.0, &reduced);
            }
            if let Some(grad_rhs) = &rhs_for_closure.grad {
                let reduced = reduce_grad(upstream_grad, &rhs_shape);
                grad_rhs.borrow_mut().scaled_add(1.0, &reduced);
            }
        });
        // --- ИСПРАВЛЕНИЕ: Оборачиваем BackwardContext в Rc::new ---
        result.ctx = Some(Rc::new(BackwardContext {
            inputs: vec![a.clone(), b.clone()],
            backward_fn,
        }));
    }
    result
}