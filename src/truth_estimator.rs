pub type Truth = f64;

pub trait TruthEstimator {
    fn get_last_estimate(&self) -> Option<Truth>;
    fn estimate(&mut self, truth: &Truth) -> Truth;
}
