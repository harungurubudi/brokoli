use crate::domain::sharedkernel::{email::Email, password::Hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
}

impl AccountRole {
    #[allow(dead_code)]
    pub fn from_str(text: &str) -> AccountRole {
        match text {
            "admin" => AccountRole::Admin,
            _ => AccountRole::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
}

impl AccountStatus {
    #[allow(dead_code)]
    pub fn from_str(text: &str) -> AccountStatus {
        match text {
            "active" => AccountStatus::Active,
            _ => AccountStatus::Deleted,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    _id: Uuid,
    email: Email,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    hash: Hash,
    role: AccountRole,
    status: AccountStatus,
    created_at: u64,
    updated_at: u64,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_serialize() {
        let my_uuid: Uuid = Uuid::from_str("61279487-2eab-406c-9265-c6985dcbc3be").unwrap();
        let now: u64 = 1669969469;
        let entity: Account = Account {
            _id: my_uuid,
            email: Email::from("harun@digitalsekuriti.id"),
            hash: Hash::from("expected_hash"),
            role: AccountRole::from_str("admin"),
            status: AccountStatus::from_str("active"),
            created_at: now,
            updated_at: now,
        };

        let serialized: String = serde_json::to_string(&entity).unwrap();
        let expected: String = String::from("{\"_id\":\"61279487-2eab-406c-9265-c6985dcbc3be\",\"email\":\"harun@digitalsekuriti.id\",\"role\":\"admin\",\"status\":\"active\",\"created_at\":1669969469,\"updated_at\":1669969469}");
        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_deserialize() {
        let payload: &str = r#"{
            "_id": "61279487-2eab-406c-9265-c6985dcbc3be",
            "email": "harun@digitalsekuriti.id",
            "hash": "123456",
            "role": "admin",
            "status": "active",
            "created_at": 1669969469,
            "updated_at": 1669969469
        }"#;

        let v: Account = serde_json::from_str(payload).unwrap();
        assert_eq!("61279487-2eab-406c-9265-c6985dcbc3be", v._id.to_string());
        assert_eq!("harun@digitalsekuriti.id", v.email.to_string());
        assert_eq!("123456", v.hash.to_string());
        assert_eq!(AccountRole::Admin, v.role);
        assert_eq!(AccountStatus::Active, v.status);
        assert_eq!(1669969469u64, v.created_at);
        assert_eq!(1669969469u64, v.updated_at);
    }
}
