use chrono::{DateTime, Utc};
use crate::{domain::ingredient::model::IngredientQueryModel, infrastructure::dtos::quantity::model::graphql::GraphQLQuantityQueryModel};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLIngredientQueryModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub unii: Option<String>,
    pub ingredient_type: Option<String>,
    pub quantity: Option<GraphQLQuantityQueryModel>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Into<IngredientQueryModel> for GraphQLIngredientQueryModel {
    fn into(self) -> IngredientQueryModel {
        IngredientQueryModel {
            id: self.id,
            name: self.name,
            unii: self.unii,
            ingredient_type: self.ingredient_type,
            quantity: self.quantity.map(|q| q.into()),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<IngredientQueryModel> for GraphQLIngredientQueryModel {
    fn from(value: IngredientQueryModel) -> Self {
        GraphQLIngredientQueryModel {
            id: value.id,
            name: value.name,
            unii: value.unii,
            ingredient_type: value.ingredient_type,
            quantity: value.quantity.map(|q| q.into()),
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
