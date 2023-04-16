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

/// Creates a new successful API response.
///
/// Will return a `200 OK` response by default.
#[macro_export]
macro_rules! ok {
    ($code:tt => $ok:expr) => {{
        use crate::api::ApiSuccess;
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
        use crate::api::ApiError;
        ApiError($err.to_string(), Some($data)).into_response(StatusCode::$code)
    }};
    ($err:ident $data:expr) => {
        err!(BAD_REQUEST => $err $data)
    };

    // No `data` field.
    ($code:tt => $err:ident) => {{
        use crate::api::ApiError;
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
