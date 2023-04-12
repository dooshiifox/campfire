use crate::prelude::*;

mod register;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register);
}
