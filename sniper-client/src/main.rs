use std::clone;

//https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
use futures::prelude::*;
use serde_json::json;
use tokio::net::UnixStream;
use tokio_serde::formats::*;
use service::SniperServiceClient;
use tarpc::{client, context, serde_transport, tokio_serde::formats::Json, transport};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

//Right now, this is just a "test" client, but planned to store functions used across client libs
#[tokio::main]
pub async fn main(){
    println!("Hello from sniper client!");
    // Bind a server socket

    let session_id="12345";
    let test_uri="test.py";
    let lang="python";
    let socket_path="/tmp/sniper.socket";
    let mut codec_builder=LengthDelimitedCodec::builder();
    let conn = UnixStream::connect(socket_path).await.unwrap();
    //SniperServiceClient;
    let transport = serde_transport::new(codec_builder.new_framed(conn), Json::default());
    let client=SniperServiceClient::new(Default::default(),transport).spawn();
    
    //let requests= async move {
        client.add_target(tarpc::context::current(),session_id.to_string(),test_uri.to_string(),lang.to_string()).await;

        let snippet=client.get_snippet(tarpc::context::current(),lang.to_string(),"if/elif/else".to_string()).await;
        
        println!("{:?}",snippet);
        
    //}
        //println!("{:?}",snippet);

    // Delimit frames using a length header
    //let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());

    // Serialize frames with JSON
    
}


