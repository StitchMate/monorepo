use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Default)]
pub struct SubsidiariesQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub company_type: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}