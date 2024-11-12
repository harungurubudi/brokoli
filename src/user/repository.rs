use super::super::sharedkernel::error;
use super::account::Account;
use super::registration::Registration;
use mockall::*;

#[automock]
pub trait AccountRepository {
    fn register(
        &self,
        registration: Registration,
    ) -> Result<Account, error::ApplicationError<'static>>;
    fn get_by_id(&self, id: &str) -> Result<Option<Account>, error::ApplicationError<'static>>;
}
