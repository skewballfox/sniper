/*
    This is a "client" built entirely for testing the functionality of the server. Eventually as the server
    becomes more developed, I'm hoping to also use this for stress testing with multiple client request going at once.
*/

use futures::TryFutureExt;
use hyper_util::rt::tokio::TokioIo;
use sniper_common::sniper_proto::{
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
    sniper_common::init_tracing("Sniper Test Client").expect("failed to initialize tracing");

    tracing::info!("Hello from sniper client!");
    let session_id = "12345".to_string();
    let test_uri = "test.py".to_string();
    let language = "python".to_string();

    let mut client = init_client("/tmp/sniper.socket".to_string()).await;
    println!("client: {:#?}", client);
    tracing::info!("client: {:#?}", client);

    //first lets add "new session" to the servers list of targets
    tracing::info!("adding target");
    let target_request = Request::new(TargetRequest {
        session_id: session_id.clone(),
        uri: test_uri.clone(),
        language,
    });

    let _ = client.add_target(target_request).await;

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
    tracing::debug!("component_stream: {:?}", component_stream);
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
            UnixStream::connect(socket_path.clone()).map_ok(TokioIo::new)
        }))
        .await
        .unwrap();

    SniperClient::new(channel)
}
