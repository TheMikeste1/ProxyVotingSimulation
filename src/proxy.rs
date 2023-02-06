use std::rc::Rc;

use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;
use crate::Delegation;

#[derive(Debug)]
pub struct Proxy {
    agent: Rc<Agent>,
    constituent_delegations: Vec<Delegation>,
    vote: f64,
}

impl Proxy {
    pub fn new(agent: Rc<Agent>) -> Self {
        let preference = agent.get_preference();
        Self {
            agent,
            constituent_delegations: Vec::default(),
            vote: preference,
        }
    }

    pub fn add_delegation(&mut self, delegation: Delegation) {
        self.constituent_delegations.push(delegation);
    }

    pub fn coordinate(
        &mut self,
        coordination_mechanism: &dyn CoordinationMechanism,
    ) -> f64 {
        self.vote =
            coordination_mechanism.coordinate(self.get_agent(), self.get_delegations());
        self.vote
    }

    pub fn get_agent(&self) -> &Agent {
        &self.agent
    }

    pub fn get_delegations(&self) -> &[Delegation] {
        &self.constituent_delegations
    }

    pub fn get_vote(&self) -> f64 {
        self.vote
    }

    pub fn get_weight(&self) -> f64 {
        (
            // +1 for the proxy's own weight
            self.constituent_delegations.len() + 1
        ) as f64
    }
}
