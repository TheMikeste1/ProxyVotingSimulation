use crate::TruthEstimator;

pub struct Ranking<'a> {
    pub proxy: Box<&'a dyn TruthEstimator>,
    /// The ranking requested by the proxy
    pub requested_ranking: u32,
    pub weight: f64,
}

impl Ranking<'_> {
    pub fn new(
        proxy: &dyn TruthEstimator,
        requested_ranking: u32,
        weight: f64,
    ) -> Ranking {
        if requested_ranking == 0 {
            panic!("requested_ranking must be greater than 0");
        }

        Ranking {
            proxy: Box::new(proxy),
            requested_ranking,
            weight,
        }
    }
}
