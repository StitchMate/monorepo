use base::Query;

#[derive(Query, Debug, Clone)]
pub enum ContainsQuery {
    ContainsById(ContainsByIdQuery)
}

#[derive(Debug, Clone)]
pub struct ContainsByIdQuery {
    pub id: String,
}

impl Into<ContainsQuery> for ContainsByIdQuery {
    fn into(self) -> ContainsQuery {
        return ContainsQuery::ContainsById(self)
    }
}