use crate::domain::quantity::model::QuantityQueryModel;

#[derive(Clone, Debug, Default)]
pub struct ContainsQueryModel {
    pub quantity: Option<QuantityQueryModel>,
    pub contains: Option<Vec<ContainsQueryModel>>,
} 