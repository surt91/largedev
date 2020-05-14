use rand::Rng;

pub trait MarkovChain {
    /// introduce a small change to propose as the next state in the chain
    fn change(&mut self, rng: &mut impl Rng);

    /// undo the previous `change`
    fn undo(&mut self);

    /// the defining value of the current state
    /// e.g. for a Metropolis algorithm on this chain, it should return the energy
    fn value(&self) -> f64;
}
