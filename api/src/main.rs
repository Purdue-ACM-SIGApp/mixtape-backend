#![feature(iterator_try_collect)]
#![feature(async_closure)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[macro_use]
extern crate dotenv_codegen;

use std::fmt::Debug;
use std::sync::Arc;

use actix::{Actor, Addr};
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{route, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use build_time::build_time_utc;
use futures_util::lock::Mutex;
use serde::Deserialize;
use serde_qs::actix::QsQueryConfig;
use serde_qs::Config;
use thiserror::Error;

use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{JsonStorageLayer, BunyanFormattingLayer};
use tracing_log::LogTracer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

mod user;
mod friend;
mod safety;
mod playlist;

#[route("/", method = "GET", method = "HEAD")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Mixtape API Server\nBuild Timestamp {}",
        build_time_utc!()
    ))
}

#[derive(Deserialize)]
struct QueryOffset {
    offset: Option<u32>,
}

#[allow(clippy::expect_used)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to enable LogTracer");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            "Mixtape".to_string(),
            std::io::stdout
        ));

    set_global_default(subscriber).expect("Failed to set global log");

    openssl_probe::init_ssl_cert_env_vars();

    HttpServer::new(move || {
        let qs_config = QsQueryConfig::default().qs_config(Config::new(2, false));

        App::new()
            .wrap(TracingLogger::default())
            .app_data(qs_config)
            .service(root)
            .configure(friend::config)
            .configure(playlist::config)
            .configure(safety::config)
            .configure(user::config)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await?;

    Ok(())
}
