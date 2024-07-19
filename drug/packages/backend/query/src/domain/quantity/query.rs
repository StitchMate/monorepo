use base::Query;

#[derive(Query, Debug, Clone)]
pub enum QuantityQuery {
    QuantityById(QuantityByIdQuery)
}

#[derive(Debug, Clone)]
pub struct QuantityByIdQuery {
    pub id: String,
}

impl Into<QuantityQuery> for QuantityByIdQuery {
    fn into(self) -> QuantityQuery {
        return QuantityQuery::QuantityById(self)
    }
}