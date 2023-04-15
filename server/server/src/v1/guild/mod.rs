use crate::v1::prelude::*;
pub mod create;

pub async fn get_guilds() -> impl Responder {
    ok!("test")
}
