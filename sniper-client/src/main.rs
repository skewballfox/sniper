use std::{clone, time::Duration};

//https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
use futures::prelude::*;
use serde_json::json;
use tokio::net::UnixStream;
use tokio_serde::formats::*;
use sniper_common::service::{SniperServiceClient, init_tracing};
use tarpc::{client, context, serde_transport, tokio_serde::formats::Json, transport};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

//Right now, this is just a "test" client, but planned to store functions used across client libs
#[tokio::main]
pub async fn main()-> anyhow::Result<()>{
    println!("Hello from sniper client!");
    // Bind a server socket
    //init_tracing("Sniper test Client")?;
    let session_id="12345";
    let test_uri="test.py";
    let lang="python";
    
    let client=sniper_common::client::init_client().await;
    
    println!("starting first request");
    //let requests= async move {
    client.add_target(tarpc::context::current(),session_id.to_string(),test_uri.to_string(),lang.to_string()).await;
    println!("sleeping");
    //std::thread::sleep(Duration::from_secs(5));
    println!("requesting snippet");
    
    let snippet=client.get_snippet(tarpc::context::current(),lang.to_string(),"if/elif/else".to_string());
    
    println!("{:?}",snippet.await);
    let triggers=client.get_triggers(tarpc::context::current(),session_id.to_string(),test_uri.to_string());
    println!("{:?}",triggers.await);

    //opentelemetry::global::shutdown_tracer_provider();
    Ok(())
    
}


