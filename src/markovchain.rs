use rand::Rng;

use crate::Model;

/// An abstract Markov Chain trait, which defines a `change` move to propose a new state
/// and a method to `undo` the last change. Also needs a primary observable defined the
/// super trait `Model`.
/// This trait can then be used to implement a range of Markov chain Monte Carlo methods,
/// like the Metropolis Algorithm or Wang Landau sampling.
pub trait MarkovChain: Model {
    /// introduce a small change to propose as the next state in the chain
    fn change(&mut self, rng: &mut impl Rng);

    /// undo the previous `change`
    fn undo(&mut self);
}
