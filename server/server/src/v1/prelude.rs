pub use crate::{
    actix_err, err, ok,
    prelude::*,
    route,
    v1::{
        authentication::{AuthMiddleware, Session},
        ApiError,
    },
};

pub const ISE: &'static str = "ISE";
pub const NOT_FOUND: &'static str = "EndpointNotFound";
pub const METHOD_NOT_ALLOWED: &'static str = "MethodNotAllowed";
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

#[macro_export]
macro_rules! route {
    ($cfg:ident : { $($route_or_method:tt => $block:tt),* $(,)? }) => {
        $(
            route!(create_services $cfg: $route_or_method => $block);
        )*

        if $(route!(has_routes $route_or_method) | )* false {
            // Routes exist
            #[allow(unused_mut)]
            let mut resource = web::resource("").default_service(
                web::route().to(|| async {
                    #[allow(unused_mut)]
                    let mut vec: Vec<String> = Vec::new();
                    $(
                        route!(create_default vec $route_or_method);
                    )*
                    err!(
                        METHOD_NOT_ALLOWED => METHOD_NOT_ALLOWED
                        "Permitted: ".to_string() + &vec.join(", ")
                    )
                })
            );

            $(
                route!(create_resource resource $route_or_method => $block);
            )*

            $cfg.service(resource);
        }
    };

    (create_services $cfg:ident : $route:literal => { $($route_or_method:tt => $block:tt),* $(,)? }) => {
        $cfg.service(web::scope($route).configure(|cfg| {
            route!(cfg: { $($route_or_method => $block),* });
        }));
    };
    (create_services $cfg:ident : $route:tt => $to:tt) => {};

    (create_resource $resource:ident $route:literal => { $($route_or_method:tt => $block:tt),* $(,)? }) => {};
    (create_resource $resource:ident $route:tt => (
        $( :( $( $middleware:expr ),* $(,)? ) )? $to:expr
    )) => {
        $resource = $resource.route(
            web::$route()
            .to($to)
            $( $( .wrap($middleware) )* )?
        );
    };

    (create_default $vec:ident $route:literal) => {};
    (create_default $vec:ident $route:tt) => {
        $vec.push(stringify!($route).to_string().to_uppercase())
    };
    (has_routes $route:literal) => { false };
    (has_routes $route:tt) => { true };
}
