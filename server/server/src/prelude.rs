pub use crate::{map, UserSnowflakeGen};
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
    // ($($key:tt => $value:expr),*) => {{
    //     let mut map = std::collections::HashMap::new();
    //     $(map.insert(stringify!($key), $value);)*
    //     map
    // }};
    ($map:ident :: $key:ident) => {{
        $map.insert(stringify!($key), $key);
    }};
    ($map:ident :: $key:ident => $value:expr) => {{
        $map.insert(stringify!($key), $value);
    }};
    ($map:ident :: $key:ident => $value:expr, $($rest:tt)*) => {{
        $map.insert(stringify!($key), $value);
        map!($map :: $($rest)*);
    }};
    ($map:ident :: $key:ident, $($rest:tt)*) => {{
        $map.insert(stringify!($key), $key);
        map!($map :: $($rest)*);
    }};
    ($($rest:tt)*) => {{
        let mut map = std::collections::HashMap::new();
        map!(map :: $($rest)*);
        map
    }};
}
