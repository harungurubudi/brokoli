use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use validator::Validate;

// pub struct Email{text: String}
#[derive(Validate, PartialEq, Eq)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    /// Mengembalikan object email dari string literal
    pub fn from(value: &str) -> Email {
        Email {
            value: String::from(value),
        }
    }
}

impl fmt::Debug for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Email { value: s })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! email_test{
        (
            $(
                ($test_name: ident, $input_text: expr, $is_valid: expr)
            ),*
        ) => {
            $(
                #[test]
                fn $test_name() {
                    let my_email:Email = Email::from($input_text);
                    assert_eq!(my_email.validate().is_err(), !$is_valid)
                }
            )*
        };
    }

    email_test! {
        (invalid_email_test, "harunasolole", false),
        (valid_email_test, "harun@digitalsekuriti.id", true)
    }
}
