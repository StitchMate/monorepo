use chrono::{DateTime, Utc};

use crate::domain::{contains::model::ContainsQueryModel, quantity::model::QuantityQueryModel};

#[derive(Clone, Debug, Default)]
pub struct PackagesQueryModel {
    pub id: Option<String>,
    pub package_ndc: String,
    pub quantity: Option<QuantityQueryModel>,
    pub contains: Option<Vec<ContainsQueryModel>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}