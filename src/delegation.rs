use std::rc::Rc;

use crate::Agent;

#[derive(Debug)]
pub struct Delegation {
    constituent: Rc<Agent>,
}

impl Delegation {
    pub fn new(constituent: Rc<Agent>) -> Self {
        Self { constituent }
    }

    pub fn get_constituent(&self) -> &Agent {
        &self.constituent
    }
}
