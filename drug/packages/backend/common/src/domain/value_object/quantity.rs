use serde::{Deserialize, Serialize};
use super::quantity_value::QuantityValue;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Quantity {
    numerator: QuantityValue,
    denominator: QuantityValue,
}

// how will this work
impl Quantity {
    pub fn new(numerator: QuantityValue, denominator: QuantityValue) -> Self {
        Self {
            numerator: numerator.into(),
            denominator: denominator.into()
        }
    }
}