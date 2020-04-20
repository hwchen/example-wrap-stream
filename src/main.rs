use futures_core::task::{Context, Poll};
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt, Tokio02AsyncWriteCompatExt};

//use futures_io::{AsyncRead, AsyncWrite};
//use tokio::io::{AsyncRead as TRead, AsyncWrite as TWrite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;

    let addr = listener.local_addr()?;
    println!("mitey serving at {}", addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let stream = stream.compat();
        let stream = WrapStream(Arc::new(stream));
    }

    Ok(())
}

/// Needed because async-std tcpstream impl Clone, but tokio tcpstream doesn't?
#[derive(Clone)]
struct WrapStream(Arc<Compat<TcpStream>>);

impl futures_io::AsyncRead for WrapStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        (*self.0).poll_read(cx, buf)
    }
}

impl futures_io::AsyncWrite for WrapStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}
