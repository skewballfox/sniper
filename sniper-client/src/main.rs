//https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html

//use sniper_common::service::SniperServiceClient;
//use tarpc::{client, context};

//this is just a "test" client, will probably rewrite the server to have actual test when the project is further along
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Bind a server socket
    //sniper_common::init_tracing("Sniper Test Client")?;
    tracing::info!("Hello from sniper client!");
    let session_id = "12345";
    let test_uri = "test.py";
    let lang = "python";

    //let client = sniper_common::client::init_client().await;

    tracing::info!("client: {:#?}", client);
    //let requests= async move {
    client
        .add_target(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            lang.to_string(),
        )
        .await;
    tracing::info!("requesting snippet");

    //tracing::info!("{:#?}", snippet.await);
    let completions = String::from("");
    let snippet_name = Vec::from("if/elif/else");
    for i in 1..snippet_name.len() + 1 {
        let completions = client
            .get_completions(
                tarpc::context::current(),
                session_id.to_string(),
                test_uri.to_string(),
                snippet_name[0..i].to_vec(),
            )
            .await;
        tracing::info!(
            "input: {:?}\ncompletions: {:#?}",
            String::from_utf8(snippet_name[0..i].to_vec()),
            completions
        );
    }
    let snippet = client
        .get_snippet(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            String::from("if/elif/else"),
        )
        .await;
    tracing::info!("Snippet: {:#?}", snippet);
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
