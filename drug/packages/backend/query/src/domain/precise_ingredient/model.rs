use chrono::{DateTime, Utc};
#[derive(Clone, Debug, Default)]
pub struct PreciseIngredientQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub unii: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}