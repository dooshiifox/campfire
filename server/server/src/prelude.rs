pub use crate::{
    actix_err,
    api::{
        authentication::{AuthMiddleware, Session},
        ApiError,
    },
    err, map, ok, route, ChannelSnowflakeGen, GuildMemberRoleSnowflakeGen, GuildMemberSnowflakeGen,
    GuildSnowflakeGen, MessageSnowflakeGen, RoleSnowflakeGen, UserSnowflakeGen,
};
pub use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, Json},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
pub use database::prelude::*;
pub use de_ref::{Deref, DerefMut};
pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;
pub use tokio::sync::Mutex;
pub use tracing::{debug, error, info, info_span, trace, warn};

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
