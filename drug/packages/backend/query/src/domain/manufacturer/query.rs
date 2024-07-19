use base::Query;

#[derive(Query, Debug, Clone)]
pub enum ManufacturerQuery {
    ManufacturerById(ManufacturerByIdQuery)
}

#[derive(Debug, Clone)]
pub struct ManufacturerByIdQuery {
    pub id: String,
}

impl Into<ManufacturerQuery> for ManufacturerByIdQuery {
    fn into(self) -> ManufacturerQuery {
        return ManufacturerQuery::ManufacturerById(self)
    }
}