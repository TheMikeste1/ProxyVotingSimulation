use crate::TruthEstimator;
use std::ops::Div;

pub fn average_truth_estimators(
    truth_estimators: &[&dyn TruthEstimator],
    weights: &[f64],
) -> f64 {
    truth_estimators
        .iter()
        .zip(weights)
        .map(|(te, w)| {
            te.get_last_estimate()
                .expect("All truth estimators should have an estimate!")
                * w
        })
        .sum::<f64>()
        .div(weights.iter().sum::<f64>())
}

#[macro_export]
macro_rules! average_truth_estimators {
    ($truth_estimators:expr) => {
        average_truth_estimators!(
            $truth_estimators,
            vec![1f64; $truth_estimators.len()].as_slice()
        )
    };

    ($truth_estimators:expr, $weights:expr) => {
        $crate::utils::average_truth_estimators($truth_estimators, $weights)
    };
}
