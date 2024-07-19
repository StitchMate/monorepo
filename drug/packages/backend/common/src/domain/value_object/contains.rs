use serde::{Deserialize, Serialize};
use super::quantity::Quantity;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Contains {
    quantity: Option<Quantity>,
    contains: Option<Vec<Contains>>,
}

// how will this work
impl Contains {
    pub fn new(quantity: Quantity, contains: Vec<Contains>) -> Self {
        Self {
            quantity: quantity.into(),
            contains: contains.into()
        }
    }
}