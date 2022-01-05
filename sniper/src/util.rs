pub mod sniper_proto {
    tonic::include_proto!("sniper");
}

pub use opentelemetry::global::shutdown_tracer_provider;
/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    println!("initializing tracer");
    std::env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;
    println!("tracer initialized {:#?}", tracer);
    tracing_subscriber::util::SubscriberInitExt::try_init(
        tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with(
            tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with(
                tracing_subscriber::registry(),
                tracing_subscriber::fmt::layer().with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::NEW
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                ),
            ),
            tracing_opentelemetry::layer().with_tracer(tracer),
        ),
    )
    .expect("error initializing tracer");
    println!("tracer registered");
    Ok(())
}
