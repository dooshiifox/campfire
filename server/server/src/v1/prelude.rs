pub use crate::{
    actix_err, err, ok,
    prelude::*,
    v1::authentication::{AuthMiddleware, Session},
};

pub const ISE: &'static str = "ISE";
pub const NO_AUTH_TOKEN: &'static str = "NoAuthToken";
pub const BAD_AUTH_TOKEN: &'static str = "BadAuthToken";
pub const INVALID_AUTH_TOKEN: &'static str = "InvalidAuthToken";
pub const JSON_PAYLOAD_TOO_LARGE: &'static str = "JSON:PayloadTooLarge";
pub const JSON_INVALID_CONTENT_TYPE: &'static str = "JSON:InvalidContentType";
pub const JSON_DESERIALIZE_ERROR: &'static str = "JSON:UnknownDeserializeError";
pub const JSON_SERIALIZE_ERROR: &'static str = "JSON:UnknownSerializeError";
pub const JSON_READING_PAYLOAD_ERROR: &'static str = "JSON:UnknownErrorReadingPayload";
pub const MISC_JSON_ERROR: &'static str = "JSON:UnknownError";

/// Creates a new successful API response.
///
/// Will return a `200 OK` response by default.
#[macro_export]
macro_rules! ok {
    ($code:tt => $ok:expr) => {{
        use crate::v1::ApiSuccess;
        ApiSuccess($ok).into_response(StatusCode::$code)
    }};
    ($ok:expr) => {
        ok!(OK => $ok)
    };
}

/// Creates a new unsuccessful API response.
///
/// Will return a `400 Bad Request` response by default.
///
/// Can optionally take a `data` field after the error string.
#[macro_export]
macro_rules! err {
    // With `data` field.
    ($code:tt => $err:ident $data:expr) => {{
        use crate::v1::ApiError;
        ApiError($err.to_string(), Some($data)).into_response(StatusCode::$code)
    }};
    ($err:ident $data:expr) => {
        err!(BAD_REQUEST => $err $data)
    };

    // No `data` field.
    ($code:tt => $err:ident) => {{
        use crate::v1::ApiError;
        ApiError::<()>($err.to_string(), None).into_response(StatusCode::$code)
    }};
    ($err:ident) => {
        err!(BAD_REQUEST => $err)
    };
}

/// Creates a new `[actix_web::error::Error]` with the given
/// error code and error message.
#[macro_export]
macro_rules! actix_err {
    // With `data` field.
    ($code:tt => $err:ident $data:expr) => {{
        let err = actix_web::error::InternalError::from_response(
            $err,
            err!($code => $err $data)
        );
        err.into()
    }};
    ($err:ident $data:expr) => {
        actix_err!(BAD_REQUEST => $err $data)
    };

    // No `data` field.
    ($code:tt => $err:ident) => {{
        let err = actix_web::error::InternalError::from_response(
            $err,
            err!($code => $err)
        );
        err.into()
    }};
    ($err:ident) => {
        actix_err!(BAD_REQUEST => $err)
    };
}
