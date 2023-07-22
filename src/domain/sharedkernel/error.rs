use std::collections::HashMap;

use serde::Serialize;
use validator::{Validate, ValidationErrorsKind};

/**
==============================================================================================================================
Berikut adalah ApplicationError (mostly http error).
==============================================================================================================================
ApplicationError dapat didefinisikan dalam bentuk macro. Macro yang tersedia adalah :
- internal_server_error
- unauthorized_error
- forbidden_error
- bad_request_error
- not_found_error
- too_many_request_error

Error macro dapat digunakan dalam tiga bentuk. :
- Default error dan description. Misal, internal_server_error!()
- Custom description, default error. Misal, internal_server_error!("Kami mengalami masalah dalam memproses permintaan anda")
- Custom description, custom error. Misal, unauthorized_error!("Sesi login anda sudah tidak berlaku", "expired_token")

Khusus untuk vallidation error akan di-define dengan melakukan wrapping dari validator. Misal :
- ApplicationError::validate(<validated object>)
*/

// Definisi status error
#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum ApplicationErrorStatus {
    InternalServerError,
    UnauthorizedError,
    ForbiddenError,
    BadRequestError,
    NotFoundError,
    TooManyRequestError,
    ValidationError,
}

// Definisi error struct
#[derive(Debug, Serialize)]
pub struct ApplicationError<'a> {
    #[serde(skip_serializing)]
    pub status: ApplicationErrorStatus,
    #[serde(skip_serializing)]
    pub code: u16,
    pub error: &'a str,
    pub description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<&'a str, Vec<String>>>,
}

impl ApplicationError<'_> {
    /*
    Berikut adalah proses wrapping validator. ValidationError dari validator akan dibentuk ulang
    sesuai dengan format dan spesifikasi error response body
    */
    pub fn validate<T: Validate>(object: T) -> Option<ApplicationError<'static>> {
        let errors: HashMap<&str, Vec<String>> = object
            .validate()
            .err()?
            .errors()
            .iter()
            .map(|error_kind| {
                (
                    *error_kind.0,
                    match error_kind.1 {
                        ValidationErrorsKind::Struct(struct_err) => {
                            validation_errs_to_str_vec(struct_err)
                        }
                        ValidationErrorsKind::Field(field_errs) => field_errs
                            .iter()
                            .map(|fe| format!("{{\"{}\": {:?}}}", fe.code, fe.params))
                            .collect(),
                        ValidationErrorsKind::List(vec_errs) => vec_errs
                            .iter()
                            .map(|ve| {
                                format!(
                                    "{}: {:?}",
                                    ve.0,
                                    validation_errs_to_str_vec(ve.1).join(" | "),
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        Some(ApplicationError {
            status: ApplicationErrorStatus::ValidationError,
            code: 400u16,
            error: "invalid_input",
            description: "Please check your input",
            fields: Some(errors),
        })
    }
}

// Membentuk kembali ValidationErrors agar sesuai dengan spesifikasi error response body
fn validation_errs_to_str_vec(ve: &validator::ValidationErrors) -> Vec<String> {
    ve.field_errors()
        .iter()
        .map(|fe| {
            format!(
                "{{\"{}\": errors: {}}}",
                fe.0,
                fe.1.iter()
                    .map(|ve| format!("{}: {:?}", ve.code, ve.params))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect()
}

impl std::error::Error for ApplicationError<'_> {}
impl std::fmt::Display for ApplicationError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

macro_rules! application_error_function {
    (
        $(
            (
                $name: ident,
                $status: ident,
                $code: expr,
                $default_error: expr,
                $default_description: expr
            )
        ),*
    ) => {
        $(
            #[allow(unused_macros)]
            macro_rules! $name {
                /*
                Definisi marco untuk membuat object error tanpa argument tambahan.
                Code, status dan description menggunakan default value
                */
                () => {
                    crate::domain::sharedkernel::error::ApplicationError{
                        status: crate::domain::sharedkernel::error::ApplicationErrorStatus::$status,
                        code: $code,
                        error: $default_error,
                        description: $default_description,
                        fields: None,
                    }
                };

                // Definisi marco untuk membuat object error dengan argument description menggantikan default description.
                ($description: expr) => {
                    crate::sharedkernel::error::ApplicationError{
                        status: crate::sharedkernel::error::ApplicationErrorStatus::$status,
                        code: $code,
                        error: $default_error,
                        description: $description,
                        fields: None,
                    }
                };

                // Definisi marco untuk membuat object error dengan argument description dan error yang menggantikan default value.
                ($description: expr, $error: expr) => {
                    crate::sharedkernel::error::ApplicationError{
                        status: crate::sharedkernel::error::ApplicationErrorStatus::$status,
                        code: $code,
                        error: $error,
                        description: $description,
                        fields: None,
                    }
                };
            }

            #[allow(unused_imports)]
            pub(crate) use $name;
        )*
    };
}

application_error_function! {
    (
        internal_server_error,
        InternalServerError,
        500,
        "internal_server_error",
        "It's not you. We are experiencing technical difficulties. Please try again later."
    ),
    (
        unauthorized_error,
        UnauthorizedError,
        401,
        "unauthorized",
        "Sorry, but you need to authenticate to access this resource."
    ),
    (
        forbidden_error,
        ForbiddenError,
        403,
        "forbidden",
        "Sorry, but you are not allowed to access this resource."
    ),
    (
        bad_request_error,
        BadRequestError,
        400,
        "bad_request",
        "Sorry, but your input is not like we are expected. Please try again."
    ),
    (
        not_found_error,
        NotFoundError,
        400,
        "not_found",
        "Sorry, but we can't find resource you are looking for."
    ),
    (
        too_many_request_error,
        TooManyRequestError,
        429,
        "too_many_request",
        "Sorry, it seems you are trying too many request. Please relax a little and try again later."
    )
}

#[cfg(test)]
mod test {
    use super::*;

    struct Expectation<'a> {
        code: u16,
        message: &'a str,
    }

    macro_rules! test_cases {
        (
            $(
                ($test_name: ident, $function_name: ident, $expected: expr)
            ),*
        ) => {
            $(
                #[test]
                fn $test_name() {
                    let error = $function_name!();
                    assert_eq!($expected.code, error.code);
                    assert_eq!($expected.message, error.error);
                }
            )*
        };
    }

    test_cases! {
        (internal_server_error_test, internal_server_error, Expectation {
            code:500, message:"internal_server_error"
        }),
        (unauthorized_error_test, unauthorized_error, Expectation{
            code: 401, message:"unauthorized"
        }),
        (forbidden_error_test, forbidden_error, Expectation{
            code: 403, message:"forbidden"
        }),
        (bad_request_error_test, bad_request_error, Expectation{
            code: 400, message:"bad_request"
        }),
        (too_many_request_error_test, too_many_request_error, Expectation{
            code: 429, message:"too_many_request"
        })
    }

    #[test]
    fn test_validation_error() {
        #[derive(validator::Validate)]
        struct MyInput<'a> {
            #[validate(length(min = 16, max = 32))]
            id: &'a str,
            #[validate(email)]
            email: &'a str,
            #[validate(length(min = 5, max = 64))]
            name: &'a str,
            #[validate(range(min = 17, max = 64))]
            age: u16,
        }

        let my_input: MyInput = MyInput {
            id: "12345678",
            email: "digitalsekrtii.co",
            name: "harun",
            age: 16,
        };

        let error = super::ApplicationError::validate(my_input).unwrap();

        assert_eq!(ApplicationErrorStatus::ValidationError, error.status);

        let fields = error.fields.unwrap();
        assert!(fields.contains_key("id"));
        assert!(fields.contains_key("email"));
        assert!(fields.contains_key("age"));
    }

    #[test]
    fn test_validation_success() {
        #[derive(validator::Validate)]
        struct MyInput<'a> {
            #[validate(length(min = 16, max = 32))]
            id: &'a str,
            #[validate(email)]
            email: &'a str,
            #[validate(length(min = 5, max = 64))]
            name: &'a str,
            #[validate(range(min = 17, max = 64))]
            age: u16,
        }

        let my_input: MyInput = MyInput {
            id: "1234567812345678",
            email: "harun@digitalsekrtii.co",
            name: "harun",
            age: 18,
        };

        let error = super::ApplicationError::validate(my_input);

        assert!(error.is_none());
    }
}
