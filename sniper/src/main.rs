mod config;

mod parser;
mod server;
mod snippet;
mod snippet_manager;
mod target;
mod util;

use config::SniperConfig;
use daemonize::Daemonize;
use dashmap::DashMap;

use snippet_manager::SnippetManager;
use tokio_serde::formats::Bincode;

use std::{os::unix::fs::DirBuilderExt, sync::Arc};

//use futures::{future, lock::Mutex, prelude::*};

use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use crate::server::Server;

#[tokio::main]
async fn main() {
    //free the socket created by old instances
    let _ = std::fs::remove_file("/tmp/sniper.socket");

    //initialize jaeger tracing
    //sniper_common::init_tracing("Sniper Server").expect("failed to initialize tracing");
    //create a lister on the specified socket
    let listener = UnixListener::bind("/tmp/sniper.socket").unwrap();

    let codec_builder = LengthDelimitedCodec::builder();

    let config = Arc::new(SniperConfig::new());
    let targets = Arc::new(DashMap::new());
    let snippets = Arc::new(DashMap::new());
    let snippet_sets = Arc::new(DashMap::new());
    let snippet_manager = SnippetManager::new(snippets.clone(), snippet_sets.clone());

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        /*let framed_stream = codec_builder.new_framed(stream);
        let transport = serde_transport::new(framed_stream, Bincode::default());

        let sniper_server = Server::new(config.clone(), targets.clone(), snippet_manager.clone());
        let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
        println!("request received");
        tokio::spawn(fut);
        */
    }
}
