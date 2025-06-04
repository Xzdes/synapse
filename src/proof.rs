use crate::{asg::ASG, proof_dsl::ProofDSL, SynapseResult};
use z3::Context;

/// Проверяет доказательства на основе ASG.
///
/// # Аргументы
///
/// * `_asg` — Абстрактный синтаксический граф (пока не используется).
///
/// # Возвращает
///
/// `SynapseResult<bool>`
pub fn check_proofs(_asg: &ASG) -> SynapseResult<bool> {
    let config = z3::Config::new();
    let context = Context::new(&config);
    let mut proof_dsl = ProofDSL::new(&context);

    proof_dsl.assert("(declare-const x Int)")?;
    proof_dsl.assert("(assert (> x 0))")?;

    proof_dsl.check()
}
