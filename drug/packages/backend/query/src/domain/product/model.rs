use chrono::{DateTime, Utc};

use crate::domain::{ingredient::model::IngredientQueryModel,
     manufacturer::model::ManufacturerQueryModel,
      package::model::PackagesQueryModel};


#[derive(Clone, Debug, Default)]
pub struct ProductQueryModel {
    pub id: Option<String>,
    pub ingredients: Option<Vec<IngredientQueryModel>>,
    pub manufacturer: Option<ManufacturerQueryModel>,
    //#[serde(rename = "generic_name")] // when deserilizing map generic_name to name
    pub name: Option<String>, // this is generic name that is country based
    pub brand_name: Option<String>,
    pub product_ndc: Option<String>,
    //#[serde(rename = "type")]
    pub product_type: Option<String>,
    pub packages: Option<Vec<PackagesQueryModel>>,
    pub form: Option<String>,
    pub route: Option<String>,
    pub schedule: Option<String>,
    pub anda: Option<String>,
    pub spl_set_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}