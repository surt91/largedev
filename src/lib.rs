use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;

use rand::Rng;


pub trait MarkovChain {
    fn change(&mut self, rng: &mut impl Rng);
    fn undo(&mut self);
    fn value(&self) -> f64;
}

pub trait Metropolis: MarkovChain {
    fn steps(&mut self, mut rng: &mut impl Rng, n: usize, temperature: f64) -> f64 {
        let mut new_energy = self.value();
        let mut old_energy;

        let beta = 1. / temperature;

        for _ in 0..n {
            self.change(&mut rng);
            old_energy = new_energy;
            new_energy = self.value();

            if ((old_energy - new_energy) * beta).exp() > rng.gen::<f64>() {
                self.undo();
                new_energy = old_energy;
            }
        }
        self.value()
    }
}

pub struct MetropolisConfig<M> {
    /// file handle of the output file
    model: M,
    /// file handle of the output file
    output: PathBuf,
    /// temperature at which to simulate
    temperature: f64,
    /// how many change moves does one sweep have
    sweep: usize,
    /// equilibration time in sweeps
    t_eq: usize,
    /// how many values to sample (total number of change moves is (`iterations` + `t_eq`) * `sweep`)
    iterations: usize,
}

impl<M: Metropolis> MetropolisConfig<M> {
    pub fn new(model: M) -> Self {
        MetropolisConfig::<M> {
            model,
            output: "out.dat".into(),
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

    pub fn output(&mut self, filename: PathBuf) -> &mut Self {
        self.output = filename;
        self
    }

    pub fn run(&mut self, mut rng: &mut impl Rng) -> io::Result<()> {
        let mut file = File::create(&self.output)?;
        writeln!(file, "# some header maybe (TODO)")?;

        let beta = 1./self.temperature;
        let mut energy_new = self.model.value();
        let mut energy_old;

        // simulate
        for i in 0..self.t_eq + self.iterations {
            for _ in 0..self.sweep {
                energy_old = energy_new;
                self.model.change(&mut rng);
                energy_new = self.model.value();

                if ((energy_old - energy_new) * beta).exp() > rng.gen::<f64>() {
                    self.model.undo();
                    energy_new = energy_old;
                }
            }

            if i > self.t_eq {
                writeln!(file, "{}", energy_new)?;
            }
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
