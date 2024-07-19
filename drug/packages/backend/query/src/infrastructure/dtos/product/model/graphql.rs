use chrono::{DateTime, Utc};
use crate::{domain::product::model::ProductQueryModel, infrastructure::dtos::{ingredient::model::graphql::GraphQLIngredientQueryModel, manufacturer::model::graphql::GraphQLManufacturerQueryModel, package::model::graphql::GraphQLPackageQueryModel}};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLProductQueryModel{
    pub id: Option<String>,
    pub ingredients: Option<Vec<GraphQLIngredientQueryModel>>,
    pub manufacturer: Option<GraphQLManufacturerQueryModel>,
    pub name: Option<String>,
    pub brand_name: Option<String>,
    pub product_ndc: Option<String>,
    pub product_type: Option<String>,
    pub packages: Option<Vec<GraphQLPackageQueryModel>>,
    pub form: Option<String>,
    pub route: Option<String>,
    pub schedule: Option<String>,
    pub anda: Option<String>,
    pub spl_set_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<ProductQueryModel> for GraphQLProductQueryModel {
    fn into(self) -> ProductQueryModel {
        ProductQueryModel {
            id: self.id,
            ingredients: self.ingredients.map(|vec| vec.into_iter().map(Into::into).collect()),
            manufacturer: self.manufacturer.map(|q| q.into()),
            brand_name: self.brand_name,
            product_ndc: self.product_ndc,
            product_type: self.product_type,
            packages: self.packages.map(|vec| vec.into_iter().map(Into::into).collect()),
            form: self.form,
            route: self.route,
            schedule: self.schedule,
            anda: self.anda,
            spl_set_id: self.spl_set_id,
            updated_at: self.updated_at,
            created_at: self.created_at,
            name: self.name,
        }
    }
}

impl From<ProductQueryModel> for GraphQLProductQueryModel {
    fn from(value: ProductQueryModel) -> Self {
        GraphQLProductQueryModel {
            id: value.id,
            ingredients: value.ingredients.map(|vec| vec.into_iter().map(Into::into).collect()),
            manufacturer: value.manufacturer.map(|q| q.into()),
            brand_name: value.brand_name,
            product_ndc: value.product_ndc,
            product_type: value.product_type,
            packages: value.packages.map(|vec| vec.into_iter().map(Into::into).collect()),
            form: value.form,
            route: value.route,
            schedule: value.schedule,
            anda: value.anda,
            spl_set_id: value.spl_set_id,
            updated_at: value.updated_at,
            created_at: value.created_at,
            name: value.name
        }
    }
}
