use std::io::{self, Write};
use std::fs::File;

use rand::Rng;

use crate::Model;

/// An trait which implements the `reconstruct` method to generate a new uniform sample
/// of the implementing model.
pub trait DirectSamplable: Model {
    fn reconstruct(&mut self, rng: &mut impl Rng);
}

/// A struct used to perform simple sampling on some model, which implements the
/// `DirectSamplable` trait. This follows the builder pattern to specify all parameters.
/// The `run` method executes the sampling, e.g.:
///
/// ```
/// let (mean, var) = Simple::new(model)
///    .iterations(1000)
///    .run(&mut rng, outfile)?;
/// ```
pub struct Simple<DS> {
    /// file handle of the output file
    model: DS,
    /// how many values to sample (total number of change moves is (`iterations` + `t_eq`) * `sweep`)
    iterations: usize,
}

impl<DS: DirectSamplable> Simple<DS> {
    pub fn new(model: DS) -> Self {
        Simple::<DS> {
            model,
            iterations: 1,
        }
    }

    pub fn iterations(&mut self, iterations: usize) -> &mut Self {
        assert!(iterations > 0);
        self.iterations = iterations;
        self
    }

    pub fn run(&mut self, mut rng: &mut impl Rng, file: &mut File) -> io::Result<(f64, f64)> {
        let mut mean = Mean::new();
        // simulate
        for _ in 0..self.iterations {
            self.model.reconstruct(&mut rng);
            let val = self.model.value();
            mean.update(val);
            writeln!(file, "{}", self.model.save())?;
        }

        let (mean, var) = mean.finalize();
        Ok((mean, var))
    }
}

/// `Mean` enables the calculation of the mean and variance on the fly without the
/// need to save all encountered values, as necessary for the naive approach.
/// It offers the `update` method to feed a new value into the mean and the `finalize`
/// method to obtain the mean and variance of all feeded values.
#[derive(Clone, Debug)]
struct Mean {
    count: u64,
    mean: f64,
    m2: f64,
}

impl Mean {
    fn new() -> Mean {
        Mean {
            count: 0,
            mean: 0.,
            m2: 0.,
        }
    }

    // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm

    // For a new value newValue, compute the new count, new mean, the new M2.
    // mean accumulates the mean of the entire dataset
    // M2 aggregates the squared distance from the mean
    // count aggregates the number of samples seen so far
    fn update(&mut self, new_value: f64) {
        self.count += 1;
        let delta = new_value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = new_value - self.mean;
        self.m2 += delta * delta2;
    }

    /// Retrieve the mean, variance and sample variance from an aggregate
    fn finalize(&self) -> (f64, f64) {
        let (mean, variance, _sample_variance) = (self.mean, self.m2 / self.count as f64, self.m2 / (self.count - 1) as f64);
        if self.count < 2 {
            panic!("too few samples")
        }

        (mean, variance)
    }
}
