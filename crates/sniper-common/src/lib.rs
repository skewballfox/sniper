use opentelemetry::global;
pub use opentelemetry::{global::shutdown_tracer_provider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime, trace as sdktrace, Resource,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

pub mod sniper_proto {
    tonic::include_proto!("sniper");
}

//TODO: flesh out different options, probably make terminal output optional
// useful docs: https://tokio.rs/tokio/topics/tracing
/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    println!("initializing tracer");
    std::env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::Config::default()
                .with_resource(Resource::new(vec![KeyValue::new(SERVICE_NAME, "sniper")])),
        )
        .install_batch(runtime::Tokio)?;
    println!("tracer initialized {:#?}", tracer_provider);

    global::set_tracer_provider(tracer_provider);
    let txt = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(txt)?;
    //global::tracer("sniper");

    println!("tracer registered");
    Ok(())
}
