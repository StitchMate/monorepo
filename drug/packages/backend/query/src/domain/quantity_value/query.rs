use base::Query;

#[derive(Query, Debug, Clone)]
pub enum QuantityValueQuery {
    QuantityValueById(QuantityValueByIdQuery)
}

#[derive(Debug, Clone)]
pub struct QuantityValueByIdQuery {
    pub id: String,
}

impl Into<QuantityValueQuery> for QuantityValueByIdQuery {
    fn into(self) -> QuantityValueQuery {
        return QuantityValueQuery::QuantityValueById(self)
    }
}