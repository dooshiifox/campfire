//! A middleware to check if a user is authenticated.

use crate::v1::prelude::*;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InnerAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct InnerAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for InnerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Decode the `Authorization: Bearer {access-token}` header
            let Some(auth_header) = req.headers().get("Authorization") else {
                return
                    Err(actix_err!(UNAUTHORIZED => NO_AUTH_TOKEN "Expected header `Authorization: Bearer {access-token}`"))
            };

            let Some(auth_token) = auth_header.to_str().ok() else {
                return
                    Err(actix_err!(UNAUTHORIZED => BAD_AUTH_TOKEN "Could not convert to string"))
            };

            let Some(token) = auth_token.strip_prefix("Bearer ") else {
                return
                    Err(actix_err!(UNAUTHORIZED => BAD_AUTH_TOKEN "Missing Bearer prefix"))
            };

            // Connect to the database and get the user id from this token
            let Some(pool) = req.app_data::<web::Data<DbPool>>() else {
                warn!("No database pool found in request");
                return Err(actix_err!(INTERNAL_SERVER_ERROR => ISE))
            };

            let (user_id, token) = match pool.access_token().check(token).await {
                Ok(data) => data,
                Err(access_token::CheckError::InvalidToken) => {
                    return Err(actix_err!(UNAUTHORIZED => INVALID_AUTH_TOKEN))
                }
                Err(access_token::CheckError::JwtDecoding(_)) => {
                    return Err(actix_err!(UNAUTHORIZED => BAD_AUTH_TOKEN "Could not decode JWT"));
                }
                Err(access_token::CheckError::DatabaseError(err)) => {
                    warn!("Database error: {}", err);
                    return Err(actix_err!(INTERNAL_SERVER_ERROR => ISE));
                }
            };

            // Add the user id to the request
            req.extensions_mut().insert(Session { user_id, token });

            // Call the next service
            let fut = service.call(req);
            let res = fut.await?;

            Ok(res)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: Snowflake,
    pub token: i64,
}

impl actix_web::FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<Session>()
                .map(|s| s.clone())
                .ok_or(actix_err!(INTERNAL_SERVER_ERROR => ISE "`Session` param not set, did you forget to wrap in an `AuthMiddleware`?")),
        )
    }
}
