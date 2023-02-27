use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((steam, _)) => {
                println!("new connection accept!");
                tokio::spawn(async move {
                    handle_connection(steam).await.unwrap();
                });
            }
            Err(e) => {
                println!("err = {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);

    loop {
        let bytes_read = stream.read_buf(&mut buf).await?;
        if bytes_read == 0 {
            println!("connection close");
            break
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}
