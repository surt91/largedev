use rand::Rng;

pub trait MarkovChain {
    /// introduce a small change to propose as the next state in the chain
    fn change(&mut self, rng: &mut impl Rng);

    /// undo the previous `change`
    fn undo(&mut self);

    /// the defining value of the current state
    /// e.g. for a Metropolis algorithm on this chain, it should return the energy
    fn value(&self) -> f64;

    /// a method which returns the string which should be saved
    /// by an algorithm operating on the markov chain
    /// by default it is `value()`, but can be overwritten
    /// if the `value()` method is expensive, this should be overwritten to
    /// return, e.g., a cached value
    fn save(&self) -> String {
        self.value().to_string()
    }
}
