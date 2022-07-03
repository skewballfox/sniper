mod config;

mod parser;
mod server;
mod snippet;
mod snippet_manager;
mod target;
mod util;

use crate::config::SniperConfig;
use crate::server::Sniper;

use crate::snippet_manager::SnippetManager;
use crate::util::sniper_proto::sniper_server::SniperServer;

use std::path::Path;
use std::sync::Arc;

//use futures::{future, lock::Mutex, prelude::*};

use futures::TryFutureExt;
use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tonic::transport::Server;
//use daemonize::Daemonize;
use dashmap::DashMap;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/sniper.socket";
    //free the socket created by old instances
    let _ = tokio::fs::remove_file(path);

    tokio::fs::create_dir_all(Path::new(path).parent().unwrap()).await?;
    //initialize jaeger tracing
    //util::init_tracing("Sniper Server").expect("failed to initialize tracing");
    //create a lister on the specified socket
    let listener = UnixListener::bind("/tmp/sniper.socket").unwrap();

    let codec_builder = LengthDelimitedCodec::builder();

    let config = Arc::new(SniperConfig::new());
    let targets = Arc::new(DashMap::new());
    let snippets = Arc::new(DashMap::new());
    let snippet_sets = Arc::new(DashMap::new());
    let snippet_manager = SnippetManager::new(snippets.clone(), snippet_sets.clone());

    let sniper = Sniper::new(config.clone(), targets.clone(), snippet_manager.clone());
    //loop {
    let (stream, _addr) = listener.accept().await.unwrap();
    /*let framed_stream = codec_builder.new_framed(stream);
    let transport = serde_transport::new(framed_stream, Bincode::default());

    let sniper_server = Server::new(config.clone(), targets.clone(), snippet_manager.clone());
    let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
    println!("request received");
    tokio::spawn(fut);
    */
    //}
    let incoming = {
        let uds = UnixListener::bind(path)?;

        async_stream::stream! {
            loop {
                let item = uds.accept().map_ok(|(st, _)| crate::util::unix::UnixStream(st)).await;

                yield item;
            }
        }
    };

    Server::builder()
        .add_service(SniperServer::new(sniper))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
