use std::process::Command;

use crate::service::SniperServiceClient;

use tarpc::serde_transport;
use tokio::net::UnixStream;
use tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

pub async fn init_client() -> SniperServiceClient {
    let mut codec_builder = LengthDelimitedCodec::builder();
    let conn = match UnixStream::connect(crate::SOCKET_PATH).await {
        Ok(it) => it,
        _ => unreachable!(),
    };
    let framed_stream = codec_builder.new_framed(conn);

    let transport = serde_transport::new(framed_stream, Bincode::default());
    SniperServiceClient::new(Default::default(), transport).spawn()
}

pub fn start_server() {
    Command::new("sniper").arg("daemonize");
}
pub use tarpc::context::current as tarpc_context;
pub use tokio;
