use base::Query;

#[derive(Query, Debug, Clone)]
pub enum PreciseIngredientQuery {
    PreciseIngredientById(PreciseIngredientByIdQuery),
    PreciseIngredientByUnii(PreciseIngredientByUniiQuery)
}

#[derive(Debug, Clone)]
pub struct PreciseIngredientByIdQuery {
    pub id: String,
}

impl Into<PreciseIngredientQuery> for PreciseIngredientByIdQuery {
    fn into(self) -> PreciseIngredientQuery {
        return PreciseIngredientQuery::PreciseIngredientById(self)
    }
}

#[derive(Debug, Clone)]
pub struct PreciseIngredientByUniiQuery {
    pub unii: String,
}

impl Into<PreciseIngredientQuery> for PreciseIngredientByUniiQuery {
    fn into(self) -> PreciseIngredientQuery {
        return PreciseIngredientQuery::PreciseIngredientByUnii(self)
    }
}