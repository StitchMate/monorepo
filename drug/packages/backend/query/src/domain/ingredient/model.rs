use chrono::{DateTime, Utc};

use crate::domain::quantity::model::QuantityQueryModel;

#[derive(Clone, Debug, Default)]
pub struct IngredientQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub unii: Option<String>,
    pub ingredient_type: Option<String>,
    pub quantity: Option<QuantityQueryModel>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
} 