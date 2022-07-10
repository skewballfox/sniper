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

use tokio::net::UnixListener;

use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
//use daemonize::Daemonize;
use dashmap::DashMap;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //initialize jaeger tracing
    util::init_tracing("Sniper Server").expect("failed to initialize tracing");

    let path = "/tmp/sniper.socket";

    let incoming = create_listener_stream(path).expect("failed to create socket");
    //create the server that will process the request
    let sniper = create_sniper_server();

    Server::builder()
        .add_service(SniperServer::new(sniper))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}

///Function to create a unix domain socket with the provided path
fn create_listener_stream(path: &str) -> Result<UnixListenerStream, std::io::Error> {
    //free the socket created by old instances
    let _ = std::fs::remove_file(path);
    //create the path to the file if it doesn't exist
    std::fs::create_dir_all(
        Path::new(path)
            .parent()
            .expect(&format!("could not get parent path from {:?}", path)),
    );
    //create a lister on the specified socket
    let listener =
        UnixListener::bind(path).expect(&format!("failed to bind Listener to path: {:?}", path));
    Ok(UnixListenerStream::new(listener))
}

fn create_sniper_server() -> Sniper {
    //will probably become more important later, right now just handles pathing
    let config = Arc::new(SniperConfig::new());
    //the individuals files, or editor sessions using sniper(still trying to figure out which)
    let targets = Arc::new(DashMap::new());
    //the snippets, it's all about the snippets
    let snippets = Arc::new(DashMap::new());
    // the set of snippets currently being used, along with the number of targets using them
    let snippet_sets = Arc::new(DashMap::new());
    // the handler of the shared state holding all data relevant to snippets
    let snippet_manager = SnippetManager::new(snippets.clone(), snippet_sets.clone());

    Sniper::new(config.clone(), targets.clone(), snippet_manager.clone())
}
