use {crate::clause::Lit, std::default::Default};

use crate::core::ExplainTheoryArg;
/// Argument passed to the Theory
pub use crate::core::TheoryArg;

/// Theory that parametrizes the solver and can react on events.
pub trait Theory {
    /// Check the model candidate `model` thoroughly.
    ///
    /// Call `acts.model()` to obtain the model.
    /// If the partial model isn't satisfiable in the theory then
    /// this *must* call `acts.raise_conflict` with a valid lemma that is
    /// the negation of a subset of the `model`.
    fn final_check(&mut self, acts: &mut TheoryArg);

    /// Push a new backtracking level
    fn create_level(&mut self);

    /// Pop `n` levels from the stack
    fn pop_levels(&mut self, n: usize);

    /// Number of levels
    fn n_levels(&self) -> usize;

    /// Check partial model (best effort).
    ///
    /// The whole partial model so far is `acts.model()`,
    /// but the theory may remember the length of the previous slice and use
    /// `acts.model()[prev_len..]` to get only the new literals.
    ///
    /// This is allowed to not raise a conflict even if the partial
    /// model is invalid, if the theory deems it too costly to verify.
    /// The model will be checked again in `final_check`.
    ///
    /// The default implementation just returns without doing anything.
    fn partial_check(&mut self, _acts: &mut TheoryArg) {}

    /// If the theory uses `TheoryArgument::propagate`, it must implement
    /// this function to explain the propagations.
    ///
    /// `p` is the literal that has been propagated by the theory in a prefix
    /// of the current trail.
    ///
    /// ## Returns
    /// - `lits` a clause that is a tautology of the theory (ie a lemma).
    ///   `lits[0]` must be `p`, and all other elements in `lits` must be false in the current model
    fn explain_propagation_clause<'a>(
        &'a mut self,
        p: Lit,
        st: &'a mut ExplainTheoryArg,
    ) -> &'a [Lit];

    /// Similar to `explain_propagation_clause` but theories should prefer larger older explanations
    /// For example, if a theory knows `(a && b) => c` and `c => d` and is asked to explain `d`,
    /// `explain_propagation_clause` may prefer to explain using `[c]` to generate a better clause,
    /// but `explain_propagation_clause_final` may as well explain using `[a, b]` since otherwise
    /// it would just be asked to explain `c` anyway.
    ///
    /// The default implementation just calls `explain_propagation`
    fn explain_propagation_clause_final<'a>(
        &'a mut self,
        p: Lit,
        st: &'a mut ExplainTheoryArg,
    ) -> &'a [Lit] {
        self.explain_propagation_clause(p, st)
    }
}

/// Trivial theory that does nothing
pub struct EmptyTheory(usize);

impl EmptyTheory {
    /// New empty theory.
    pub fn new() -> Self {
        EmptyTheory(0)
    }
}

impl Default for EmptyTheory {
    fn default() -> Self {
        EmptyTheory::new()
    }
}

// theory for any context.
impl Theory for EmptyTheory {
    fn final_check(&mut self, _: &mut TheoryArg) {}
    fn create_level(&mut self) {
        self.0 += 1
    }
    fn pop_levels(&mut self, n: usize) {
        debug_assert!(self.0 >= n);
        self.0 -= n
    }
    fn n_levels(&self) -> usize {
        self.0
    }
    fn explain_propagation_clause(&mut self, _p: Lit, _: &mut ExplainTheoryArg) -> &[Lit] {
        unreachable!()
    }
}
