mod simple;
pub use simple::{DirectSamplable, Simple};

mod markovchain;
pub use markovchain::MarkovChain;

mod metropolis;
pub use metropolis::Metropolis;

/// The fundamental trait of any model, which defines at least one observable to measure
pub trait Model {
    /// the defining value of the current state
    /// e.g. for a Metropolis algorithm on this chain, it should return the energy
    fn value(&self) -> f64;

    /// a method which returns the string which should be saved
    /// for each sample by default it is `value()`, but can be overwritten
    /// to save multiple observables per sample
    /// if the `value()` method is expensive, this should be overwritten to
    /// return, e.g., a cached value
    fn save(&self) -> String {
        self.value().to_string()
    }
}
