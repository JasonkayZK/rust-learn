use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    println!("server listening...");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("Connecting to address: {:?}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has closed
                    Ok(0) => {
                        println!("data send ok");
                        TcpStream::shutdown(&mut socket).await.unwrap();
                        println!("connect closed");
                        return
                    },
                    Ok(n) => {
                        // Copy the data back to socket
                        println!("Read from socket: {:#?}", String::from_utf8(buf[..n].to_owned()));
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(e) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        eprintln!("Error reading from socket: {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
