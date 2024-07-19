use chrono::{DateTime, Utc};
 
use crate::{domain::package::model::PackagesQueryModel, infrastructure::dtos::{contains::model::graphql::GraphQLContainsQueryModel, quantity::model::graphql::GraphQLQuantityQueryModel}};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLPackageQueryModel {
    pub id: Option<String>,
    pub package_ndc: String,
    pub quantity: Option<GraphQLQuantityQueryModel>,
    pub contains: Option<Vec<GraphQLContainsQueryModel>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<PackagesQueryModel> for GraphQLPackageQueryModel {
    fn into(self) -> PackagesQueryModel {
        PackagesQueryModel {
            id: self.id,
            package_ndc: self.package_ndc,
            quantity: self.quantity.map(|q| q.into()),
            contains: self.contains.map(|vec| vec.into_iter().map(Into::into).collect()),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<PackagesQueryModel> for GraphQLPackageQueryModel {
    fn from(value: PackagesQueryModel) -> Self {
        GraphQLPackageQueryModel {
            id: value.id,
            package_ndc: value.package_ndc,
            quantity: value.quantity.map(|q| q.into()),
            contains: value.contains.map(|vec| vec.into_iter().map(Into::into).collect()),
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
