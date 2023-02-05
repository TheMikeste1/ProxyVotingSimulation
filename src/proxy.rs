use std::rc::Rc;

use crate::Agent;
use crate::Delegation;

#[derive(Debug)]
pub struct Proxy {
    agent: Rc<Agent>,
    constituent_delegations: Vec<Delegation>,
}

impl Proxy {
    pub fn new(agent: Rc<Agent>) -> Self {
        Self {
            agent,
            constituent_delegations: Vec::default(),
        }
    }

    pub fn add_delegation(&mut self, delegation: Delegation) {
        self.constituent_delegations.push(delegation);
    }

    pub fn coordinate(&self) -> f64 {
        // TODO: STUB
        0.0
    }

    pub fn get_agent(&self) -> &Agent {
        &self.agent
    }

    pub fn get_delegations(&self) -> &Vec<Delegation> {
        &self.constituent_delegations
    }
}
