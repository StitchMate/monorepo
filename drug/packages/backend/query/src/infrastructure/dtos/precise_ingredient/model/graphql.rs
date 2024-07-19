use chrono::{DateTime, Utc};
use crate::domain::precise_ingredient::model::PreciseIngredientQueryModel;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLPreciseIngredientQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub unii: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<PreciseIngredientQueryModel> for GraphQLPreciseIngredientQueryModel {
    fn into(self) -> PreciseIngredientQueryModel {
        PreciseIngredientQueryModel {
            id: self.id,
            name: self.name,
            unii: self.unii,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<PreciseIngredientQueryModel> for GraphQLPreciseIngredientQueryModel {
    fn from(value: PreciseIngredientQueryModel) -> Self {
        GraphQLPreciseIngredientQueryModel {
            id: value.id,
            name: value.name,
            unii: value.unii,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
