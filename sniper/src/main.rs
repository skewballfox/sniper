mod config;
mod snippet;
mod snippet_manager;
mod target;

mod sniper_server;

use config::SniperConfig;
use dashmap::DashMap;
use sniper_common::service::{init_tracing, SniperService};
use snippet_manager::SnippetManager;
use tokio_serde::formats::Bincode;
//mod server;
//mod server.rs

use std::{path::PathBuf, sync::Arc};

use futures::{future, lock::Mutex, prelude::*};
use tarpc::{
    serde_transport,
    server::{self, Channel, Incoming},
};
use tokio::net::UnixListener; //, UnixStream};
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use crate::sniper_server::SniperServer;

#[tokio::main] //(flavor = "current_thread")]
async fn main() {
    let _ = std::fs::remove_file(sniper_common::SOCKET_PATH);
    let listener = UnixListener::bind(sniper_common::SOCKET_PATH).unwrap();

    let codec_builder = LengthDelimitedCodec::builder();

    let config = Arc::new(SniperConfig::new());
    let targets = Arc::new(DashMap::new());
    let snippets = Arc::new(DashMap::new());
    let snippet_sets = Arc::new(DashMap::new());
    let snippet_manager = SnippetManager::new(snippets.clone(), snippet_sets.clone());

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let framed_stream = codec_builder.new_framed(stream);
        let transport = serde_transport::new(framed_stream, Bincode::default());

        let sniper_server =
            SniperServer::new(config.clone(), targets.clone(), snippet_manager.clone());
        let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
        println!("request recieved");
        tokio::spawn(fut);
    }
}
//init_tracing("Sniper Server");
/*tokio::spawn(async move {
    let sniper_server = SniperServer::new(config.clone(), targets.clone(), sniper.clone());
    let session_id = "00000";
    let test_uri = "test.py";
    let lang = "python";
    &sniper_server
        .clone()
        .add_target(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            lang.to_string(),
        )
        .await;

    sniper_server
        .get_snippet(
            tarpc::context::current(),
            session_id.to_string(),
            test_uri.to_string(),
            String::from("if/elif/else"),
        )
        .await;
})
.await;
*/
