
use crate::service::SniperServiceClient;
use tarpc::serde_transport;
use tokio::net::UnixStream;
use tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

#[cfg(feature="client")]
pub async fn init_client()-> SniperServiceClient {
    let socket_path="/tmp/sniper.socket";
    let mut codec_builder=LengthDelimitedCodec::builder();
    let conn = UnixStream::connect(crate::SOCKET_PATH).await.unwrap();
    let framed_stream=codec_builder.new_framed(conn);
    
    let transport = serde_transport::new(framed_stream, Json::default());
    SniperServiceClient::new(Default::default(),transport).spawn()
}