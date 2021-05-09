
mod config;
mod target;
mod sniper;
mod snippet;
mod snippetParser;
mod rifle;
mod handler;


use service::SniperService;
//mod server;
//mod server.rs

use std::{path::{ PathBuf}, sync::Arc};

use futures::{future, lock::Mutex,prelude::*};
use handler::ConnectionHandler;
use tarpc::{serde_transport, server::{self, Incoming, Channel}, tokio_serde::formats::Json};
use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;



#[tokio::main]
async fn main()  {
    let socket_path=PathBuf::from("/tmp/sniper.socket");
    let _ = std::fs::remove_file(socket_path.clone());
    let listener = UnixListener::bind(socket_path).unwrap();
    
    let mut codec_builder=LengthDelimitedCodec::builder();
    
    let sniper=Arc::new(Mutex::new(sniper::Sniper::new()));
    tokio::spawn(async move{
        loop {
            let (stream,_addr)=listener.accept().await.unwrap();
            let framed_stream= codec_builder.new_framed(stream);
            let transport = serde_transport::new(framed_stream,Json::default());
            let sniper_server=ConnectionHandler::new(sniper.clone());
            let fut = server::BaseChannel::with_defaults(transport).execute(sniper_server.serve());
            tokio::spawn(fut);
        }
    });
    
    


    
    
    //sniper.add_target("12345","test.py","python");
    //println!("{:#?}",sniper.rifle.snippets);
    //println!("{:?}",sniper.config.languages["python"]);
    //println!("{:#?}",sniper.snipe("python","if/elif/else"));
    //println!("{:#?}",sniper.snipe("python","if"));
    //sniper.add_target("12345","test.py","python");
    
    //println!("{:?}",config)
    //let config=SniperConfig::new("")
    //println!("Hello, world!");
    //.await?;
   
}
