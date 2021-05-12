
mod config;
mod target;
mod sniper;
mod snippet;
mod snippetParser;
mod rifle;
mod handler;


use service::{init_tracing,SniperService};
//mod server;
//mod server.rs

use std::{path::{ PathBuf}, sync::Arc};

use futures::{future, lock::Mutex,prelude::*};
use handler::ConnectionHandler;
use tarpc::{serde_transport, server::{self, Incoming, Channel}, tokio_serde::formats::Json};
use tokio::net::{UnixListener, UnixStream};
use tokio_util::codec::length_delimited::LengthDelimitedCodec;



#[tokio::main]//(flavor="current_thread")]
async fn main()  -> anyhow::Result<()>{
    
    let socket_path=PathBuf::from("/tmp/sniper.socket");
    let _ = std::fs::remove_file(socket_path.clone());
    let listener = UnixListener::bind(socket_path).unwrap();
    
    let mut codec_builder=LengthDelimitedCodec::builder();
    
    let sniper=Arc::new(Mutex::new(sniper::Sniper::new()));
    
    //init_tracing("Sniper Server");
    loop {
        let (stream,_addr)=listener.accept().await.unwrap();
        let framed_stream= codec_builder.new_framed(stream);
        let transport = serde_transport::new(framed_stream,Json::default());
        let sniper_server=ConnectionHandler::new(sniper.clone());
        let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
        println!("processing first request");
        tokio::spawn(fut).await;
    }
    
    
    Ok(())
   
}
