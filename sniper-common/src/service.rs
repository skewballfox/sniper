use std::env;

use futures::{
    future::{self, Ready},
    prelude::*,
};
pub use qp_trie::Trie;
use tarpc::{
    client, context,
    server::{self, Incoming},
};
use tracing_subscriber::{
    fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnippetInfo {
    pub name: String,
    pub description: String,
}
#[tarpc::service]
pub trait SniperService {
    //NOTE: may simplify the api once client is implemented, such as removing
    //get_triggers, and making triggers a return value for add/drop_target/lib

    /// add a target to the list of currently tracked targets
    async fn add_target(session_id: String, uri: String, language: String);

    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    async fn drop_target(session_id: String, uri: String, language: String);

    ///add a snippet set to the given target
    //async fn target_add_libs(session_id: String, uri: String, libs: Vec<String>);

    ///drop a snippet set for a given target
    //async fn target_drop_libs(session_id: String, uri: String, libs: Vec<String>);

    /// get the triggers for the snippets associated with a given target
    async fn get_completions(session_id: String, uri: String, input: Vec<u8>) -> Vec<SnippetInfo>;

    /// get a snippet
    async fn get_snippet(
        session_id: String,
        uri: String,
        snippet_key: String,
    ) -> Option<Vec<String>>;
}

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}
