//https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
use futures::prelude::*;
use serde_json::json;
use tokio::net::UnixStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

//Right now, this is just a "test" client, but planned to store functions used across client libs
#[tokio::main]
pub async fn main() {
    println!("Hello from sniper client!");
    // Bind a server socket

    let session_id="12345";
    let test_uri="test.py";
    let lang="python";

    let socket = UnixStream::connect("sniper.socket").await.unwrap();

    // Delimit frames using a length header
    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());

    // Serialize frames with JSON
    let mut serialized =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

    //start_sniper(session_id,test_uri,lang);
    // Send the value
    serialized
        .send(json!({
            "session_id": "12345",
            "uri": "test.py",
            "lang": "python",
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }))
        .await
        .unwrap()
}


///either connect to existing sniper session or start sniper session
fn start_sniper<S>(session_id: S,test_uri: S, language: S) where S: Into<String> {
    
    println!("todo");
    
    //good artist copy, great artist steal
    //https://github.com/kak-lsp/kak-lsp/blob/master/src/main.rs#L209
    
}
