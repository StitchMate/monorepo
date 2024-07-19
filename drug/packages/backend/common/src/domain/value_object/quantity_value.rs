use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct QuantityValue {
    value: f64,
    unit: String,
}

impl QuantityValue {
    pub fn new<S: Into<String>>(value: f64, unit: S) -> Self {
        Self {
            value: value.into(),
            unit: unit.into()
        }
    }
}