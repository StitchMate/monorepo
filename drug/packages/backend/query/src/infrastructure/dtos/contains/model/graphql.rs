use crate::domain::contains::model::ContainsQueryModel;
use crate::infrastructure::dtos::quantity::model::graphql::GraphQLQuantityQueryModel;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct GraphQLContainsQueryModel {
    pub quantity: Option<GraphQLQuantityQueryModel>,
    pub contains: Option<Vec<GraphQLContainsQueryModel>>,
}

impl Into<ContainsQueryModel> for GraphQLContainsQueryModel {
    fn into(self) -> ContainsQueryModel {
        ContainsQueryModel {
            quantity: self.quantity.map(|q| q.into()),
            contains: self.contains.map(|vec| vec.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<ContainsQueryModel> for GraphQLContainsQueryModel {
    fn from(value: ContainsQueryModel) -> Self {
        GraphQLContainsQueryModel {
            quantity: value.quantity.map(|q| q.into()),
            contains: value.contains.map(|vec| vec.into_iter().map(Into::into).collect())
        }
    }
}
