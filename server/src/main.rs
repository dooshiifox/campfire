#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(try_trait_v2_yeet)]

pub mod database;
pub mod prelude;
mod v1;

use crate::prelude::*;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path(std::path::Path::new(".env")).unwrap();
    let _guard = init_tracing(); // Hold file guard until end of program

    let pool = database::new_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DbPool(pool.clone())))
            .service(web::scope("/v1").configure(v1::init_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    // clear the file contents
    let file = std::fs::File::create("./log.txt").expect("Unable to create log file");
    file.set_len(0).expect("Unable to clear log file");

    let file_appender = tracing_appender::rolling::never("./", "log.txt");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    use tracing_subscriber::layer::SubscriberExt;
    let mut file_writer_subscriber = tracing_subscriber::fmt::Layer::default();
    file_writer_subscriber.set_ansi(false);
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            // subscriber configuration
            .with_env_filter(&dotenvy::var("LOG_LEVEL").unwrap_or("info,server=trace".to_string()))
            .finish()
            // add additional writers
            .with(file_writer_subscriber.with_writer(file_writer)),
    )
    .expect("Unable to set global tracing subscriber");

    debug!("Tracing initialized.");
    guard
}
