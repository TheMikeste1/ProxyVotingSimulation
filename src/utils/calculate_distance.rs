use crate::TruthEstimator;

pub fn calculate_distance(a: &dyn TruthEstimator, b: &dyn TruthEstimator) -> f64 {
    a.get_last_estimate().expect("a has no estimate")
        - b.get_last_estimate().expect("b has no estimate")
}
