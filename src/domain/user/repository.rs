use super::registration::Registration;
use super::account::Account;
use crate::domain::sharedkernel::error;
use mockall::*;

#[automock]
pub trait AccountRepository {
    fn register(&self, registration: Registration) -> Result<Account, error::ApplicationError<'static>>;
    fn get_by_id(&self, id: &str) -> Result<Option<Account>, error::ApplicationError<'static>>;
}