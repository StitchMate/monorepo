use chrono::{DateTime, Utc};
use crate::{domain::manufacturer::model::ManufacturerQueryModel, infrastructure::dtos::subsidiaries::model::graphql::GraphQLSubsidiariesQueryModel};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLManufacturerQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub subsidiaries: Option<Vec<GraphQLSubsidiariesQueryModel>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<ManufacturerQueryModel> for GraphQLManufacturerQueryModel {
    fn into(self) -> ManufacturerQueryModel {
        ManufacturerQueryModel {
            id: self.id,
            name: self.name,
            subsidiaries: self.subsidiaries.map(|vec| vec.into_iter().map(Into::into).collect()),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<ManufacturerQueryModel> for GraphQLManufacturerQueryModel {
    fn from(value: ManufacturerQueryModel) -> Self {
        GraphQLManufacturerQueryModel {
            id: value.id,
            name: value.name,
            subsidiaries: value.subsidiaries.map(|vec| vec.into_iter().map(Into::into).collect()),
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
