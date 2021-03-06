mod simple;
pub use simple::{DirectSamplable, Simple};

mod markovchain;
pub use markovchain::MarkovChain;

mod histogram;
pub use histogram::Histogram;

mod metropolis;
pub use metropolis::Metropolis;

mod wanglandau;
pub use wanglandau::WangLandau;

/// The fundamental trait of any model, which defines at least one observable to measure
pub trait Model {
    /// the defining value of the current state
    /// e.g. for a Metropolis algorithm on this chain, it should return the energy
    fn value(&self) -> f64;

    /// return the header for the values returned in `save`
    fn header(&self) -> String {
        "# value".into()
    }

    /// a method which returns the string which should be saved
    /// for each sample by default it is `value()`, but can be overwritten
    /// to save multiple observables per sample
    /// if the `value()` method is expensive, this should be overwritten to
    /// return, e.g., a cached value
    fn save(&self) -> String {
        self.value().to_string()
    }
}
