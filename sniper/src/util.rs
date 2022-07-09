pub use opentelemetry::global::shutdown_tracer_provider;
use tonic::{codegen::futures_core, Status};


pub mod sniper_proto {
    tonic::include_proto!("sniper");
}

pub(crate) type Stream<T> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = std::result::Result<T, Status>> + Send>>;
// This is only temporary so that the elephant is edible
/*
/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    println!("initializing tracer");
    std::env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;
    println!("tracer initialized {:#?}", tracer);
    tracing_subscriber::util::SubscriberInitExt::try_init(
        tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with(
            tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with(
                tracing_subscriber::registry(),
                tracing_subscriber::fmt::layer().with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::NEW
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                ),
            ),
            tracing_opentelemetry::layer().with_tracer(tracer),
        ),
    )
    .expect("error initializing tracer");
    println!("tracer registered");
    Ok(())
}
*/
//TODO: remove after merge of related PR
//https://github.com/hyperium/tonic/pull/861
#[cfg(unix)]
pub mod unix {
    use std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct UnixStream(pub tokio::net::UnixStream);

    impl Connected for UnixStream {
        type ConnectInfo = UdsConnectInfo;

        fn connect_info(&self) -> Self::ConnectInfo {
            UdsConnectInfo {
                peer_addr: self.0.peer_addr().ok().map(Arc::new),
                peer_cred: self.0.peer_cred().ok(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct UdsConnectInfo {
        pub peer_addr: Option<Arc<tokio::net::unix::SocketAddr>>,
        pub peer_cred: Option<tokio::net::unix::UCred>,
    }

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}
