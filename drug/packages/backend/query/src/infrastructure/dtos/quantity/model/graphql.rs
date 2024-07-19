use chrono::{DateTime, Utc};
use crate::infrastructure::dtos::quantity_value::model::graphql::GraphQLQuantityValueQueryModel;
use crate::domain::quantity::model::QuantityQueryModel;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLQuantityQueryModel {
    pub numerator: Option<GraphQLQuantityValueQueryModel>,
    pub denominator: Option<GraphQLQuantityValueQueryModel>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<QuantityQueryModel> for GraphQLQuantityQueryModel {
    fn into(self) -> QuantityQueryModel {
        QuantityQueryModel {
            numerator: self.numerator.map(|q| q.into()),
            denominator: self.denominator.map(|q| q.into()),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<QuantityQueryModel> for GraphQLQuantityQueryModel {
    fn from(value: QuantityQueryModel) -> Self {
        GraphQLQuantityQueryModel {
            numerator: value.numerator.map(|q| q.into()),
            denominator: value.denominator.map(|q| q.into()),
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
