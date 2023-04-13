use crate::prelude::*;

mod login;
mod register;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register).service(login::login);
}
