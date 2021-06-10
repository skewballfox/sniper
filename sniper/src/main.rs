mod config;
mod snippet;
mod snippet_manager;
mod target;

mod sniper_server;

use config::SniperConfig;
use daemonize::Daemonize;
use dashmap::DashMap;
use sniper_common::{service::SniperService, shutdown_tracer_provider};
use snippet_manager::SnippetManager;
use tokio_serde::formats::Bincode;

use std::{os::unix::fs::DirBuilderExt, sync::Arc};

//use futures::{future, lock::Mutex, prelude::*};
use tarpc::{
    serde_transport,
    server::{self, Channel, Incoming},
};
use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use crate::sniper_server::SniperServer;

#[tokio::main]
async fn main() {
    //store stdout/stderr in files
    let stdout = std::fs::File::create("/tmp/sniper.out").unwrap();
    let stderr = std::fs::File::create("/tmp/sniper.err").unwrap();

    //free the socket created by old instances
    let _ = std::fs::remove_file(sniper_common::SOCKET_PATH);
    let daemon = Daemonize::new()
        .pid_file("/tmp/sniper.pid")
        //.chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(std::env::current_dir().unwrap()) // for default behaviour.
        //.user("nobody")
        //.group("sniper") // Group name
        //.group(2) // or group id.
        //.umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/sniper.out`.
        .stderr(stderr)
        //.privileged_action(|| UnixListener::bind(sniper_common::SOCKET_PATH).unwrap()) // Redirect stderr to `/tmp/sniper.err`.
        .exit_action(|| shutdown_tracer_provider())
        .start()
        .expect("failed to daemonize server");

    //initialize jaeger tracing
    sniper_common::init_tracing("Sniper Server").expect("failed to initialize tracing");
    //create a lister on the specified socket
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

pub fn temp_dir() -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push("sniper.pid");
    let old_mask = unsafe { libc::umask(0) };
    // Ignoring possible error during $TMPDIR/kak-lsp creation to have a chance to restore umask.
    let _ = std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o1777)
        .create(&path);
    unsafe {
        libc::umask(old_mask);
    }
    path.push(whoami::username());
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&path)
        .unwrap();
    path
}
