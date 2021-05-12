
mod config;
mod target;
mod sniper;
mod snippet;

mod handler;


use config::SniperConfig;
use dashmap::DashMap;
use service::{init_tracing,SniperService};
use sniper::Sniper;
//mod server;
//mod server.rs

use std::{path::{PathBuf}, sync::Arc};

use futures::{future, lock::Mutex,prelude::*};
use tarpc::{serde_transport, server::{self, Incoming, Channel}, tokio_serde::formats::Json};
use tokio::net::{UnixListener};//, UnixStream};
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use crate::handler::Spotter;



#[tokio::main]//(flavor="current_thread")]
async fn main() {
    
    let socket_path=PathBuf::from("/tmp/sniper.socket");
    let _ = std::fs::remove_file(socket_path.clone());
    let listener = UnixListener::bind(socket_path).unwrap();
    
    let mut codec_builder=LengthDelimitedCodec::builder();
    
    let config=Arc::new(Mutex::new(SniperConfig::new()));
    let targets=Arc::new(DashMap::new());
    let sniper=Arc::new(tokio::sync::RwLock::new(Sniper::new()));
    //init_tracing("Sniper Server");
    loop {
        let (stream,_addr)=listener.accept().await.unwrap();
        let framed_stream= codec_builder.new_framed(stream);
        let transport = serde_transport::new(framed_stream,Json::default());
        let sniper_server = Spotter::new(config.clone(),targets.clone(),sniper.clone());
        let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
        println!("processing request");
        tokio::spawn(fut).await;
    }
    
    
   
}
