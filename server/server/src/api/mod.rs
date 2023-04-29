mod account;
pub mod authentication;
pub mod channel;
mod guild;
mod message;
pub mod result;

use crate::prelude::*;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let json_config =
        web::JsonConfig::default()
            .limit(32768)
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

    cfg.app_data(json_config);

    cfg.default_service(web::route().to(|| async {
        #[allow(unused_mut)]
        let mut vec: Vec<String> = Vec::new();
        vec.push(stringify!(get).to_string().to_uppercase());
        err!(NOT_FOUND => NOT_FOUND "Check your spelling!")
    }));

    route!(cfg: {
        "/account" => {
            "/login" => {
                post => (account::login::login),
            },
            "/register" => {
                post => (account::register::register),
            },
        },
        "/guild" => {
            "/create" => {
                post => (:(AuthMiddleware) guild::create::create),
            },
            "/get_joined" => {
                get => (:(AuthMiddleware) guild::get_joined::get_joined),
            },
        },
        "/channel/{guild_id}" => {
            "/create" => {
                post => (:(AuthMiddleware) channel::create::create),
            }
        },
        "/message/{channel_id}" => {
            post => (:(AuthMiddleware) message::send::send),
            get => (:(AuthMiddleware) message::get::get),
        },
    });
}
