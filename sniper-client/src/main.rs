//https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html

use sniper_common::service::{init_tracing, SniperServiceClient, Trie};
use tarpc::{client, context, serde_transport, tokio_serde::formats::Json, transport};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

//Right now, this is just a "test" client, but planned to store functions used across client libs
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    println!("Hello from sniper client!");
    // Bind a server socket
    //init_tracing("Sniper test Client")?;
    let session_id = "12345";
    let test_uri = "test.py";
    let lang = "python";

    let client = sniper_common::client::init_client().await;

    println!("client: {:#?}", client);
    //let requests= async move {
    client
        .add_target(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            lang.to_string(),
        )
        .await;
    //println!("sleeping");
    //std::thread::sleep(Duration::from_secs(5));
    println!("requesting snippet");

    //println!("{:#?}", snippet.await);
    let completions = String::from("");
    let snippet_name = Vec::from("if/elif/else");
    for i in 1..snippet_name.len() + 1 {
        let completions = client.get_completions(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            snippet_name[0..i].to_vec(),
        );
        println!(
            "input: {:?}\ncompletions: {:#?}",
            String::from_utf8(snippet_name[0..i].to_vec()),
            completions.await
        );
    }
    let snippet = client.get_snippet(
        tarpc::context::current(),
        session_id.to_string(),
        test_uri.to_string(),
        String::from("if/elif/else"),
    );
    println!("Snippet: {:#?}", snippet.await);
    //opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
