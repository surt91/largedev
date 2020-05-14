use rand::Rng;

use crate::Model;

pub trait MarkovChain: Model {
    /// introduce a small change to propose as the next state in the chain
    fn change(&mut self, rng: &mut impl Rng);

    /// undo the previous `change`
    fn undo(&mut self);
}
