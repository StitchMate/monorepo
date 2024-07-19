use chrono::{DateTime, Utc};
use crate::domain::subsidiaries::model::SubsidiariesQueryModel;

#[derive(Clone, Debug, Default)]
pub struct ManufacturerQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub subsidiaries: Option<Vec<SubsidiariesQueryModel>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}