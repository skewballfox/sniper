use std::env;

use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

pub mod service;

#[cfg(feature = "client")]
pub mod client;

pub const SOCKET_PATH: &str = "/tmp/sniper.socket";
pub use opentelemetry::global::shutdown_tracer_provider;
/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    println!("initializing tracer");
    env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;
    println!("tracer initialized {:#?}", tracer);
    tracing_subscriber::registry()
        //.with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()
        .expect("error initializing tracer");
    println!("tracer registered");
    Ok(())
}
