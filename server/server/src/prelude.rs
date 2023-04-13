pub use crate::{actix_err, err, map, ok, UserSnowflakeGen};
pub use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, Json},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
pub use database::prelude::*;
// pub use database::DbPool;
pub use de_ref::{Deref, DerefMut};
pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;
pub use tokio::sync::Mutex;
pub use tracing::{debug, error, info, info_span, trace, warn};

/// Creates a new successful API response.
///
/// Will return a `200 OK` response by default.
#[macro_export]
macro_rules! ok {
    ($code:expr => $ok:expr) => {{
        use crate::v1::ApiSuccess;
        ApiSuccess($ok).into_response($code)
    }};
    ($ok:expr) => {
        ok!(StatusCode::OK => $ok)
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
    ($code:expr => $err:ident $data:expr) => {{
        use crate::v1::ApiError;
        ApiError($err.to_string(), Some($data)).into_response($code)
    }};
    ($err:ident $data:expr) => {
        err!(StatusCode::BAD_REQUEST => $err $data)
    };

    // No `data` field.
    ($code:expr => $err:ident) => {{
        use crate::v1::ApiError;
        ApiError::<()>($err.to_string(), None).into_response($code)
    }};
    ($err:ident) => {
        err!(StatusCode::BAD_REQUEST => $err)
    };
}

/// Creates a new `[actix_web::error::Error]` with the given
/// error code and error message.
#[macro_export]
macro_rules! actix_err {
    // With `data` field.
    ($code:expr => $err:ident $data:expr) => {{
        let err = actix_web::error::InternalError::from_response(
            $err,
            err!($code => $err $data)
        );
        err.into()
    }};
    ($err:ident $data:expr) => {
        actix_err!(StatusCode::BAD_REQUEST => $err $data)
    };

    // No `data` field.
    ($code:expr => $err:ident) => {{
        let err = actix_web::error::InternalError::from_response(
            $err,
            err!($code => $err)
        );
        err.into()
    }};
    ($err:ident) => {
        actix_err!(StatusCode::BAD_REQUEST => $err)
    };
}

#[macro_export]
/// Shorthand for creating a hashmap.
///
/// # Example
/// ```
/// let new_map = map!{
///     key1 => "value1",
///     key2 => "value2"
/// };
///
/// assert_eq!(new_map, std::collections::HashMap::from_iter(&[
///    ("key1", "value1"),
///    ("key2", "value2")
/// ]));
/// ```
macro_rules! map {
    ($($key:tt => $value:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert(stringify!($key), $value);)*
        map
    }};
}
