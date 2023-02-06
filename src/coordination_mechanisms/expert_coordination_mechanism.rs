use crate::{Agent, Delegation};

pub trait ExpertCoordinationMechanism {
    fn coordinate(&self, expert: &Agent, _delegations: &[Delegation]) -> f64 {
        expert.get_preference()
    }
}
