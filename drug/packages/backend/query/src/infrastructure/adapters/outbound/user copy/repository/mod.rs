use enum_dispatch::enum_dispatch;

#[cfg(test)]
use self::postgres::MockDgraphUserQueryRepository;
use self::postgres::DgraphUserQueryRepository;

use crate::application::ports::outbound::repository::UserQueryRepository;
use crate::domain::user::error::repository::UserQueryRepositoryError;
use crate::domain::user::model::UserQueryModel;
use chrono::DateTime;
use chrono::Utc;
use user_common::domain::value_object::email::Email;
use user_common::domain::value_object::name::Name;

pub mod postgres;

#[enum_dispatch(UserQueryRepository)]
#[derive(Debug)]
pub enum UserQueryRepositoryEnum {
    DgraphUserQueryRepository(DgraphUserQueryRepository),
    #[cfg(test)]
    MockDgraphUserQueryRepository(MockDgraphUserQueryRepository),
}

#[cfg(test)]
mod tests {}
