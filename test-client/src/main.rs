/*
    This is a "client" built entirely for testing the functionality of the server. Eventually as the server
    becomes more developed, I'm hoping to also use this for stress testing with multiple client request going at once.
*/
pub mod sniper_proto {
    tonic::include_proto!("sniper");
}
use crate::sniper_proto::{
    sniper_client::SniperClient, CompletionsRequest, SnippetRequest, TargetRequest,
};
use tokio::net::UnixStream;
use tonic::{
    transport::{Channel, Endpoint, Uri},
    Request,
};
use tower::service_fn;

//this is just a "test" client, will probably rewrite the server to have actual test when the project is further along
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Bind a server socket
    init_tracing("Sniper Test Client")?;

    tracing::info!("Hello from sniper client!");
    let session_id = "12345".to_string();
    let test_uri = "test.py".to_string();
    let language = "python".to_string();

    let mut client = init_client("/tmp/sniper.socket".to_string()).await;

    tracing::info!("client: {:#?}", client);

    //first lets add "new session" to the servers list of targets
    tracing::info!("adding target");
    let target_request = Request::new(TargetRequest {
        session_id: session_id.clone(),
        uri: test_uri.clone(),
        language,
    });

    client.add_target(target_request).await;

    //tracing::info!("{:#?}", snippet.await);

    let snippet_name = Vec::from("if/elif/else");

    //then we use the current user input(keystrokes) to widdle down
    //the relevant snippets until the user chooses one
    for i in 1..snippet_name.len() + 1 {
        let completions_request = Request::new(CompletionsRequest {
            session_id: session_id.clone(),
            uri: test_uri.clone(),
            user_input: snippet_name[0..i].to_vec(),
        });
        let completions = client.get_completions(completions_request).await;
        tracing::info!(
            "input: {:?}\ncompletions: {:#?}",
            String::from_utf8(snippet_name[0..i].to_vec()),
            completions
        );
    }
    tracing::info!("requesting snippet");
    //lastly we request the user selected snippet
    let snippet_request = Request::new(SnippetRequest {
        session_id,
        uri: test_uri,
        snippet_name: String::from_utf8(snippet_name).unwrap(),
    }); //TODO: consider changing the snippet name to a Vec<u8>
    let mut component_stream = client.get_snippet(snippet_request).await?.into_inner();
    while let Some(snippet_component) = component_stream.message().await? {
        tracing::info!("Snippet Component: {:#?}", snippet_component);
    }

    //tracing::info!("Snippet: {:#?}", snippet);

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

pub async fn init_client(socket_path: String) -> SniperClient<Channel> {
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            UnixStream::connect(socket_path.clone())
        }))
        .await
        .unwrap();
    SniperClient::new(channel)
}
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
