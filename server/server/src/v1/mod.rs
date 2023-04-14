mod account;
pub mod authentication;
pub mod prelude;

use crate::v1::prelude::*;
use actix_web::HttpResponseBuilder;
use std::fmt::Debug;

/// A failed API response. Optionally returns a JSON object with more information.
#[derive(Debug)]
pub struct ApiError<T: Serialize + Debug>(pub String, pub Option<T>);

impl<T: Serialize + Debug> ApiError<T> {
    pub fn into_response(self, code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(code).json(self)
    }
}

impl<T: Serialize + Debug> Serialize for ApiError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        if let Some(t) = &self.1 {
            let mut resp = serializer.serialize_struct("Error", 3).unwrap();
            resp.serialize_field("error", &true).unwrap();
            resp.serialize_field("code", &self.0).unwrap();
            resp.serialize_field("data", &t).unwrap();
            resp.end()
        } else {
            let mut resp = serializer.serialize_struct("Error", 2).unwrap();
            resp.serialize_field("error", &true).unwrap();
            resp.serialize_field("code", &self.0).unwrap();
            resp.end()
        }
    }
}

/// A successful API response.
pub struct ApiSuccess<T: Serialize>(T);

impl<T: Serialize + Debug> ApiSuccess<T> {
    pub fn into_response(self, code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(code).json(self)
    }
}

impl<T: Serialize> Serialize for ApiSuccess<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut resp = serializer.serialize_struct("Success", 2).unwrap();
        resp.serialize_field("error", &false).unwrap();
        resp.serialize_field("data", &self.0).unwrap();
        resp.end()
    }
}

#[get("/", wrap = "AuthMiddleware")]
async fn index(session: Session) -> impl Responder {
    format!("Hello! {}", session.user_id)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| match err {
            actix_web::error::JsonPayloadError::OverflowKnownLength { length, limit } => {
                actix_err!(PAYLOAD_TOO_LARGE => JSON_PAYLOAD_TOO_LARGE map!{ length, limit })
            }
            actix_web::error::JsonPayloadError::Overflow { limit } => {
                actix_err!(PAYLOAD_TOO_LARGE => JSON_PAYLOAD_TOO_LARGE map!{ limit })
            }
            actix_web::error::JsonPayloadError::ContentType => {
                actix_err!(UNSUPPORTED_MEDIA_TYPE => JSON_INVALID_CONTENT_TYPE)
            }
            actix_web::error::JsonPayloadError::Deserialize(e) => {
                actix_err!(BAD_REQUEST => JSON_DESERIALIZE_ERROR e.to_string())
            }
            actix_web::error::JsonPayloadError::Serialize(e) => {
                actix_err!(INTERNAL_SERVER_ERROR => JSON_SERIALIZE_ERROR e.to_string())
            }
            actix_web::error::JsonPayloadError::Payload(e) => {
                actix_err!(INTERNAL_SERVER_ERROR => JSON_READING_PAYLOAD_ERROR e.to_string())
            }
            _ => {
                actix_err!(INTERNAL_SERVER_ERROR => MISC_JSON_ERROR)
            }
        });

    cfg.app_data(json_config)
        .service(index)
        .service(web::scope("/account").configure(account::init_routes));
}
