use std::io::{self, Write};
use std::fs::File;

use crate::histogram::Histogram;
use crate::markovchain::MarkovChain;

use rand::Rng;


/// A struct used to perform Wang-Landau sampling on some model, which implements the
/// `MarkovChain` trait. This follows the builder pattern to specify all parameters.
/// The `run` method executes the sampling, e.g.:
///
/// ```
/// let (tries, rejects) = WangLandau::new(model)
///    .bins(WangLandau::uniform_bins(low, high, num))
///    .sweep(100)
///    .lnf_final(1e-5)
///    .run(&mut rng, outfile)?;
/// ```
pub struct WangLandau<MC> {
    /// file handle of the output file
    model: MC,
    /// lower bound for the energy of the sampled window
    low: f64,
    /// upper bound for the energy of the sampled window
    high: f64,
    /// estimate of the density of states
    g: Histogram,
    /// auxiliary histogram for flatness criterion
    h: Histogram,
    /// how many change attempts per sweep
    sweep: usize,
    /// final refinement parameter (logarithmic)
    lnf_final: f64,
}

impl<MC: MarkovChain> WangLandau<MC> {
    pub fn new(model: MC, low: f64, high: f64) -> Self {
        WangLandau::<MC> {
            model,
            low,
            high,
            g: Histogram::new(low, high, 100),
            h: Histogram::new(low, high, 100),
            sweep: 1,
            lnf_final: 1e-5,
        }
    }

    pub fn sweep(&mut self, sweep: usize) -> &mut Self {
        assert!(sweep > 0);
        self.sweep = sweep;
        self
    }

    pub fn lnf_final(&mut self, lnf_final: f64) -> &mut Self {
        assert!(lnf_final > 0.);
        self.lnf_final = lnf_final;
        self
    }

    pub fn bins(&mut self, bins: usize) -> &mut Self {
        self.g = Histogram::new(self.low, self.high, bins);
        self.h = Histogram::new(self.low, self.high, bins);
        self
    }

    /// Create a starrting walk with lb < S < ub by a simple downhill strategy.
    fn find_start(&mut self, mut rng: impl Rng) {
        loop {
            let old_e = self.model.value();
            self.model.change(&mut rng);
            let new_e = self.model.value();

            if (new_e < self.low && old_e > new_e) || (new_e > self.high && old_e < new_e) {
                self.model.undo();
            }

            if new_e > self.low && new_e < self.high {
                break;
            }
        }
    }

    fn accept(&mut self, old_e: f64, rng: &mut impl Rng) -> f64 {
        let mut new_e = self.model.value();

        let p_acc = match (self.g.at(old_e), self.g.at(new_e)) {
            (Some(old), Some(new)) => (old - new).exp(),
            // if one of the values is outside of the histogram range,
            // reject the proposal (-> p_acc = 0)
            _ => 0.,
        };

        if p_acc < rng.gen::<f64>() {
            self.model.undo();
            new_e = old_e;
        }

        new_e
    }

    /** Implementation of the "Fast" 1/t Wang Landau algorithm extended by Entropic Sampling.
     *
     * Larger values of the final refinement parameter are ok, since
     * the simulation will be "corrected" by an entropic sampling
     * simulation after the Wang Landau estimation of g.
     *
     * Literature used:
     *   * 10.1103/PhysRevE.75.046701 (original paper)
     *   * 10.1063/1.2803061 (analytical)
     *   * http://arxiv.org/pdf/cond-mat/0701672.pdf ("fast")
     *   * http://arxiv.org/pdf/1107.2951v1.pdf (entropic sampling)
     */
    #[allow(clippy::float_cmp)]
    pub fn run(&mut self, mut rng: &mut impl Rng, file: &mut File) -> io::Result<(usize, usize)> {
        let mut tries = 0;
        let mut rejects = 0;
        let initial_num_iterations = 1000;

        self.find_start(&mut rng);

        let mut t = 0;
        let mut lnf = 1.;

        // start first phase
        while t < 10 || lnf > 1./t as f64 {
            // TODO: good logging system
            println!("ln f = {}, t = {}", lnf, t);
            while self.h.min() == 0. {
                for _ in 0..initial_num_iterations {
                    for _ in 0..self.sweep {
                        let old_e = self.model.value();
                        self.model.change(&mut rng);
                        let new_e = self.accept(old_e, &mut rng);

                        tries += 1;
                        rejects += if new_e == old_e {1} else {0};

                        self.g.add(new_e, lnf);
                        self.h.count(new_e);
                    }
                    t += 1;
                }

                // emergency abort: if too much of the time is spend in this stage,
                // panic, trim the histogram and proceed
                // this might lead to inaccurate results
                if lnf == 1. && self.lnf_final > 0.2 /t as f64 {
                    println!("Spend 20% time in phase 1 at lnf=1: panic, trim the histogram and proceed");
                    println!("The results of this simulation may be inaccurate");
                    println!("You should restart with a different range or smaller lnf");
                    self.g.trim();
                    self.h.trim();
                    println!("g = {:?}", self.g);
                    println!("h = {:?}", self.h);
                    assert_eq!(self.g.bounds(), self.h.bounds());
                    lnf = self.lnf_final;
                    break;
                }
            }
            // run until we have one entry in each bin
            self.h.reset();
            lnf /= 2.;
        }

        if lnf <= self.lnf_final {
            println!("phase 1 took too long, phase 2 will not be performed");
            println!("The results of this simulation may be inaccurate");
            println!("You should restart with a different range, smaller windows or smaller lnf");
        }

        //start second phase
        // let status = 1./t as f64;
        println!("begin phase 2 (power-law decrease) at t = {}", t);
        while lnf > self.lnf_final {
            lnf = 1./t as f64;

            for _ in 0..self.sweep {
                let old_e = self.model.value();
                self.model.change(&mut rng);
                let new_e = self.accept(old_e, &mut rng);

                tries += 1;
                rejects += if new_e == old_e {1} else {0};

                self.g.add(new_e, lnf);
            }
            t += 1;
        }

        // perform entropic sampling with the bias g
        // this way the errors caused by too large f_final
        // are mitigated

        // the entropic sampling phase should be twice as long as
        // the previous phase
        println!("begin phase 3 (entropic sampling) at t = {} until t = {}", t, 3*t);
        let t_limit = 2*t;
        for _ in 0..t_limit {
            for _ in 0..self.sweep {
                let old_e = self.model.value();
                self.model.change(&mut rng);
                let new_e = self.accept(old_e, &mut rng);

                tries += 1;
                rejects += if new_e == old_e {1} else {0};

                self.h.count(new_e);
            }
            // write out samples for correlation
            // TODO
        }

        // remove the bias
        for j in 0..self.g.bins() {
            *self.g.idx(j) += *self.h.idx(j)/self.h.mean();
        }

        let centers = self.g.centers();
        let data = self.g.data();

        for (c, d) in centers.iter().zip(data) {
            writeln!(file, "{} {}\n", c, d)?;
        }

        Ok((tries, rejects))
    }
}
