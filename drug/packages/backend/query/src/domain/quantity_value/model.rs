use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Default)]
pub struct QuantityValueQueryModel {
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
} 