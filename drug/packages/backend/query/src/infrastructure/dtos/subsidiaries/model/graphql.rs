use chrono::{DateTime, Utc};  
use crate::domain::subsidiaries::model::SubsidiariesQueryModel;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLSubsidiariesQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub company_type: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
} 

impl Into<SubsidiariesQueryModel> for GraphQLSubsidiariesQueryModel {
    fn into(self) -> SubsidiariesQueryModel {
        SubsidiariesQueryModel {
            id: self.id,
            name: self.name,
            company_type: self.company_type,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<SubsidiariesQueryModel> for GraphQLSubsidiariesQueryModel {
    fn from(value: SubsidiariesQueryModel) -> Self {
        GraphQLSubsidiariesQueryModel {
            id: value.id,
            name: value.name,
            company_type: value.company_type,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
