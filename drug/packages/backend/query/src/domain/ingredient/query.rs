use base::Query;

#[derive(Query, Debug, Clone)]
pub enum IngredientQuery {
    IngredientById(IngredientByIdQuery),
    IngredientByUnii(IngredientByUniiQuery)
}

#[derive(Debug, Clone)]
pub struct IngredientByIdQuery {
    pub id: String,
}

impl Into<IngredientQuery> for IngredientByIdQuery {
    fn into(self) -> IngredientQuery {
        return IngredientQuery::IngredientById(self)
    }
}

#[derive(Debug, Clone)]
pub struct IngredientByUniiQuery {
    pub unii: String,
}

impl Into<IngredientQuery> for IngredientByUniiQuery {
    fn into(self) -> IngredientQuery {
        return IngredientQuery::IngredientByUnii(self)
    }
}