use rand::Rng;

pub trait MarkovChain {
    fn change(&mut self, rng: &mut impl Rng);
    fn undo(&mut self);
    fn value(&self) -> f64;
}
