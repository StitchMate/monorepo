use base::Query;

#[derive(Query, Debug, Clone)]
pub enum PackageQuery {
    PackageById(PackageByIdQuery)
}

#[derive(Debug, Clone)]
pub struct PackageByIdQuery {
    pub id: String,
}

impl Into<PackageQuery> for PackageByIdQuery {
    fn into(self) -> PackageQuery {
        return PackageQuery::PackageById(self)
    }
}