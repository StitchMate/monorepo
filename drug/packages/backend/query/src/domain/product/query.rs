use base::Query;

#[derive(Query, Debug, Clone)]
pub enum ProductQuery {
    ProductById(ProductByIdQuery)
}

#[derive(Debug, Clone)]
pub struct ProductByIdQuery {
    pub id: String,
}

impl Into<ProductQuery> for ProductByIdQuery {
    fn into(self) -> ProductQuery {
        return ProductQuery::ProductById(self)
    }
}