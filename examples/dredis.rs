use std::io;

use anyhow::Result;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr_port = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr_port).await?;
    info!("listen on {}", addr_port);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("accept from {:?}", addr);
        tokio::spawn(async move {
            if let Err(e) = process_client(stream).await {
                warn!("stream err:{}", e);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}
async fn process_client(mut stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                info!("recv:{}", String::from_utf8_lossy(&buf[..n]));
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
