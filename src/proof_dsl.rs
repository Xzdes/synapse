use crate::SynapseResult;
use z3::{Context, SatResult, Solver};

/// DSL для построения и проверки доказательств в Synapse.
///
/// Оборачивает контекст и солвер Z3.
pub struct ProofDSL<'ctx> {
    /// Контекст Z3.
    pub context: &'ctx Context,
    /// Солвер Z3.
    pub solver: Solver<'ctx>,
}

impl<'ctx> ProofDSL<'ctx> {
    /// Создает новый DSL для проверки доказательств.
    ///
    /// # Аргументы
    ///
    /// * `context` — Контекст Z3.
    ///
    /// # Возвращает
    ///
    /// `ProofDSL`
    pub fn new(context: &'ctx Context) -> Self {
        let solver = Solver::new(context);
        Self { context, solver }
    }

    /// Добавляет утверждение в солвер.
    ///
    /// # Аргументы
    ///
    /// * `expression` — строка с SMT-LIB выражением.
    ///
    /// # Возвращает
    ///
    /// `SynapseResult<()>`
    pub fn assert(&mut self, _expression: &str) -> SynapseResult<()> {
        // TODO: Добавить полноценный парсер SMT-LIB.
        Ok(())
    }

    /// Проверяет доказательства.
    ///
    /// # Возвращает
    ///
    /// `SynapseResult<bool>`
    pub fn check(&self) -> SynapseResult<bool> {
        let result = self.solver.check();
        Ok(result == SatResult::Sat)
    }
}
