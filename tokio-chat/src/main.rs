use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;


const LOCAL_SERVER: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(LOCAL_SERVER).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("{} connected", addr);

        tokio::spawn(async move {//task

            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut msg = String::new();
            loop {
                let bytes_read = reader.read_line(&mut msg).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                println!("{}", msg);
                writer.write_all(msg.as_bytes()).await.unwrap();
                msg.clear();
            }
        });
    }

    // Ok(())
}
