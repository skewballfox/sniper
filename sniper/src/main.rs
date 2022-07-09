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
    //create the path to the file if it doesn't exist
    tokio::fs::create_dir_all(Path::new(path).parent().unwrap()).await?;
    //create a lister on the specified socket
    let listener = UnixListener::bind(path).unwrap();

    //initialize jaeger tracing
    util::init_tracing("Sniper Server").expect("failed to initialize tracing");

    //will probably become more important later, right now just handles pathing
    let config = Arc::new(SniperConfig::new());
    //the individuals files, or editor sessions using sniper(still trying to figure out which)
    let targets = Arc::new(DashMap::new());
    //the snippets, it's all about the snippets
    let snippets = Arc::new(DashMap::new());
    // the set of snippets currently being used, along with the number of targets using them
    let snippet_sets = Arc::new(DashMap::new());
    let snippet_manager = SnippetManager::new(snippets.clone(), snippet_sets.clone());

    let sniper = Sniper::new(config.clone(), targets.clone(), snippet_manager.clone());

    let (_stream, _addr) = listener.accept().await.unwrap();

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
