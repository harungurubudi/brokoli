use super::super::sharedkernel::{email::Email, password::Password};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Registration {
    #[validate]
    email: Email,
    #[validate]
    password: Password,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {
        let payload: &str = r#"{
            "email": "harun@digitalsekuriti.id",
            "password":"1234qweR!"
        }"#;

        let v: Registration = serde_json::from_str(payload).unwrap();
        assert_eq!(
            String::from("harun@digitalsekuriti.id"),
            v.email.to_string()
        );
        assert_eq!(String::from("1234qweR!"), v.password.to_string());
    }
}
