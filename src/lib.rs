mod simple;
pub use simple::{DirectSamplable, Simple};

mod markovchain;
pub use markovchain::MarkovChain;

mod metropolis;
pub use metropolis::Metropolis;

pub trait Model {
    /// the defining value of the current state
    /// e.g. for a Metropolis algorithm on this chain, it should return the energy
    fn value(&self) -> f64;

    /// a method which returns the string which should be saved
    /// for each sample
    /// by default it is `value()`, but can be overwritten
    /// if the `value()` method is expensive, this should be overwritten to
    /// return, e.g., a cached value
    fn save(&self) -> String {
        self.value().to_string()
    }
}
