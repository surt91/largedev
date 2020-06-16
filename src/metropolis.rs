use std::io::{self, Write};
use std::fs::File;

use crate::markovchain::MarkovChain;

use rand::Rng;

/// A struct used to perform Metropolis sampling on some model, which implements the
/// `MarkovChain` trait. This follows the builder pattern to specify all parameters.
/// The `run` method executes the sampling, e.g.:
///
/// ```
/// let (tries, rejects) = Metropolis::new(model)
///    .temperature(2.269)
///    .sweep(100)
///    .iterations(1000)
///    .run(&mut rng, outfile)?;
/// ```
pub struct Metropolis<MC> {
    /// file handle of the output file
    model: MC,
    /// temperature at which to simulate
    temperature: f64,
    /// how many change moves does one sweep have
    sweep: usize,
    /// equilibration time in sweeps
    t_eq: usize,
    /// how many values to sample (total number of change moves is (`iterations` + `t_eq`) * `sweep`)
    iterations: usize,
}

impl<MC: MarkovChain> Metropolis<MC> {
    pub fn new(model: MC) -> Self {
        Metropolis::<MC> {
            model,
            temperature: 1e10,
            t_eq: 0,
            sweep: 1,
            iterations: 1,
        }
    }

    pub fn temperature(&mut self, t: f64) -> &mut Self {
        self.temperature = t;
        self
    }

    pub fn t_eq(&mut self, t_eq: usize) -> &mut Self {
        self.t_eq = t_eq;
        self
    }

    pub fn sweep(&mut self, sweep: usize) -> &mut Self {
        assert!(sweep > 0);
        self.sweep = sweep;
        self
    }

    pub fn iterations(&mut self, iterations: usize) -> &mut Self {
        assert!(iterations > 0);
        self.iterations = iterations;
        self
    }

    pub fn run(&mut self, mut rng: &mut impl Rng, file: &mut File) -> io::Result<(usize, usize)> {
        let mut tries = 0;
        let mut rejects = 0;

        let beta = 1./self.temperature;
        let mut energy_new = self.model.value();
        let mut energy_old;

        // simulate
        for i in 0..self.t_eq + self.iterations {
            for _ in 0..self.sweep {
                energy_old = energy_new;
                self.model.change(&mut rng);
                tries += 1;
                energy_new = self.model.value();

                let p_acc = ((energy_old - energy_new) * beta).exp();
                if p_acc < rng.gen_range(0., 1.) {
                    self.model.undo();
                    rejects += 1;
                    energy_new = energy_old;
                }
            }

            if i > self.t_eq {
                writeln!(file, "{}", self.model.save())?;
            }
        }

        Ok((tries, rejects))
    }

    pub fn downhill(&mut self, mut rng: &mut impl Rng) -> f64 {
        let mut energy_new = self.model.value();
        let mut energy_old;

        // simulate
        for _ in 0..self.iterations {
            energy_old = energy_new;
            self.model.change(&mut rng);
            energy_new = self.model.value();

            if energy_old > energy_new {
                self.model.undo();
                energy_new = energy_old;
            }
        }
        energy_new
    }

    pub fn uphill(&mut self, mut rng: &mut impl Rng) -> f64 {
        let mut energy_new = self.model.value();
        let mut energy_old;

        // simulate
        for _ in 0..self.iterations {
            energy_old = energy_new;
            self.model.change(&mut rng);
            energy_new = self.model.value();

            if energy_old < energy_new {
                self.model.undo();
                energy_new = energy_old;
            }
        }
        energy_new
    }
}
