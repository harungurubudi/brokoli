use super::error;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use validator::{Validate, ValidationError};

/// Merepresentasikan object password
#[derive(Validate, PartialEq, Eq)]
pub struct Password {
    #[validate(length(min = 8, max = 18), custom = "validate_pass")]
    value: String,
}

// Extended password validator
fn validate_pass(passw: &String) -> Result<(), ValidationError> {
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_number = false;
    let mut has_special = false;

    for c in passw.chars() {
        if c.is_ascii() {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    has_lower = true;
                } else if c.is_uppercase() {
                    has_upper = true;
                }
            } else if c.is_numeric() {
                has_number = true;
            } else {
                has_special = true;
            }
        }
    }

    if !(has_lower && has_upper && has_number && has_special) {
        return Err(ValidationError::new("Invalid Password"));
    }

    Ok(())
}

impl Password {
    /**
    Mengembalikan sebuah Password object dari string literal (*str)

    # Arguments
    * `value` - Sebuah literal sebagai teks password
    */
    pub fn from(value: &str) -> Password {
        Password {
            value: String::from(value),
        }
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Password { value: s })
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Merepresentasikan object hash. Biasanya digunakan untuk menyimpan hashed password.
#[derive(Default)]
pub struct Hash {
    hash: String,
}

impl Hash {
    /// Mengembalikan sebuah hash object kosong
    pub fn new() -> Hash {
        Hash {
            hash: String::new(),
        }
    }

    /**
    Mengembalikan sebuah hash dari sebuah hashed string dalam tipe string literal(&str)

    # Argumnents
    * `hash` - hashed string dalam tipe string literal
    */
    pub fn from(hash: &str) -> Hash {
        Hash {
            hash: String::from(hash),
        }
    }

    /**
    Mengembalikan hash result dari object &Password

    # Arguments
    * `key` = hashing key dalam *str format
    * `password` = reference dari object Password
    */
    pub fn from_password(
        key: &str,
        password: &Password,
    ) -> Result<Hash, error::ApplicationError<'static>> {
        match sha512_crypt::hash_with(key, password.to_string()) {
            Ok(result) => Ok(Hash { hash: result }),
            Err(_) => Err(error::internal_server_error!()),
        }
    }

    /**
    Memverifikasi object &Password apakah matched dengan Hash object

    # Arguments
    * `password` = referenc dari object Password
    */
    pub fn verify_password(&self, password: &Password) -> Result<bool, &str> {
        if self.hash.is_empty() {
            return Err("Hash is empty");
        }
        Ok(sha512_crypt::verify(password.to_string(), &self.hash))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.hash)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Hash { hash: s })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_with_empty_hash() {
        let password_value: &str = "Aasolole123!";
        let my_password: Password = Password::from(&password_value);
        let my_hash: Hash = Hash::new();

        match my_hash.verify_password(&my_password) {
            Ok(_) => {}
            Err(err) => {
                assert_eq!(false, err.is_empty())
            }
        }
    }

    #[test]
    fn test_hash_with_matched_password() {
        let password_value: &str = "Aasolole123!";
        let my_password: Password = Password::from(&password_value);
        let key: &str ="$6$G/gkPn17kHYo0gTF$xhDFU0QYExdMH2ghOWKrrVtu1BuTpNMSJURCXk43.EYekmK8iwV6RNqftUUC8mqDel1J7m3JEbUkbu4YyqSyv/";

        match Hash::from_password(key, &my_password) {
            Ok(hash) => match hash.verify_password(&my_password) {
                Ok(is_valid) => {
                    assert_eq!(true, is_valid)
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }

    macro_rules! password_validation_test_cases {
        (
            $(
                ($test_name: ident, $passw: expr, $is_err: expr)
            ),*
        ) => {
            $(
                #[test]
                fn $test_name() {
                    let password_value: &str = $passw;
                    let my_password:Password = Password::from(&password_value);
                    assert_eq!($is_err, my_password.validate().is_err())
                }
            )*
        };
    }

    password_validation_test_cases! {
        (too_short_password_test, "mypass", true),
        (lowercase_only_password_test, "mypassword", true),
        (lower_and_upper_case_only_password_test, "MypassworD", true),
        (no_special_char_password_test, "MypassworD1234", true),
        (good_password_test, "MypassworD1234!", false)
    }
}
