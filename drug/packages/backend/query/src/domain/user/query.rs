use base::Query;

#[derive(Query, Debug, Clone)]
pub enum UserQuery {
    UserById(UserByIdQuery)
}

#[derive(Debug, Clone)]
pub struct UserByIdQuery {
    pub id: String,
}

impl Into<UserQuery> for UserByIdQuery {
    fn into(self) -> UserQuery {
        return UserQuery::UserById(self)
    }
}