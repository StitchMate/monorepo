use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use mockall::automock;
use postgres::MockPostgresPackagesQueryRepository;

use crate::{application::ports::outbound::package_repository::{PackageQueryRepository}, domain::package::error::repository::PackageQueryRepositoryError};

mod postgres;

#[enum_dispatch(PackagesQueryRepository)]
#[derive(Debug)]
pub enum PackageQueryRepositoryEnum {
    MockPostgresPackageQueryRepository(MockPostgresPackagesQueryRepository),
}

#[cfg(test)]
mod tests {}
