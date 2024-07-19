use chrono::{DateTime, Utc};
use crate::domain::quantity_value::model::QuantityValueQueryModel;


#[derive(Clone, Debug, Default)]
pub struct QuantityQueryModel {
    pub numerator: Option<QuantityValueQueryModel>,
    pub denominator: Option<QuantityValueQueryModel>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
} 