use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use crate::domain::quantity_value::model::QuantityValueQueryModel;

#[derive(SimpleObject, Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLQuantityValueQueryModel {
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<QuantityValueQueryModel> for GraphQLQuantityValueQueryModel {
    fn into(self) -> QuantityValueQueryModel {
        QuantityValueQueryModel {
            value: self.value,
            unit: self.unit,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<QuantityValueQueryModel> for GraphQLQuantityValueQueryModel {
    fn from(value: QuantityValueQueryModel) -> Self {
        GraphQLQuantityValueQueryModel {
            value: value.value,
            unit: value.unit,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
