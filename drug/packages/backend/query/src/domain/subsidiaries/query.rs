use base::Query;

#[derive(Query, Debug, Clone)]
pub enum SubsidiariesQuery {
    SubsidiariesById(SubsidiariesByIdQuery)
}

#[derive(Debug, Clone)]
pub struct SubsidiariesByIdQuery {
    pub id: String,
}

impl Into<SubsidiariesQuery> for SubsidiariesByIdQuery {
    fn into(self) -> SubsidiariesQuery {
        return SubsidiariesQuery::SubsidiariesById(self)
    }
}