use ordered_float::NotNan;

#[derive(Clone, Debug)]
pub struct Histogram {
    low: f64,
    high: f64,
    bins: usize,
    histogram: Vec<f64>,
}

impl Histogram {
    pub fn new(low: f64, high: f64, bins: usize) -> Histogram {
        assert!(low < high);
        Histogram {
            low,
            high,
            bins,
            histogram: vec![0.; bins+1],
        }
    }

    pub fn add(&mut self, value: f64, amount: f64) {
        if value > self.low && value <= self.high {
            self.histogram[((value-self.low)/(self.high - self.low) * self.bins as f64) as usize] += amount;
        }
    }

    pub fn count(&mut self, value: f64) {
        if value > self.low && value <= self.high {
            self.histogram[((value-self.low)/(self.high - self.low) * self.bins as f64) as usize] += 1.;
        }
    }

    pub fn min(&self) -> f64 {
        self.histogram.iter().map(|x| NotNan::new(*x).unwrap()).min().unwrap().into_inner()
    }

    pub fn at(&self, value: f64) -> Option<f64> {
        if value > self.low && value <= self.high {
            let idx = ((value-self.low)/(self.high - self.low) * self.bins as f64) as usize;
            Some(self.histogram[idx])
        } else {
            None
        }
    }

    pub fn idx(&mut self, idx: usize) -> &mut f64 {
        &mut self.histogram[idx]
    }

    pub fn reset(&mut self) {
        for i in &mut self.histogram {
            *i = 0.;
        }
    }

    pub fn bins(&self) -> usize {
        self.bins
    }

    pub fn mean(&self) -> f64 {
        self.histogram.iter().sum::<f64>() / self.bins as f64
    }

    pub fn hist(&self) -> Vec<(f64, f64)> {
        self.histogram.iter()
            .enumerate()
            .map(|(n, &x)| (
                    (n as f64 / self.bins as f64) * (self.high - self.low) + self.low,
                    x
                )
            )
            .collect()
    }

    fn left_border(&self, n: usize) -> f64 {
        (n as f64 / self.bins as f64) * (self.high - self.low) + self.low
    }

    fn right_border(&self, n: usize) -> f64 {
        ((n+1) as f64 / self.bins as f64) * (self.high - self.low) + self.low
    }

    pub fn centers(&self) -> Vec<f64> {
        (0..self.bins).map(|i| (self.left_border(i) + self.right_border(i))/2.).collect()
    }

    pub fn data(&self) -> &[f64] {
        &self.histogram
    }
}
