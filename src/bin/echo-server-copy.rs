use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("Connecting to address: {:?}", addr);

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            // This will blocked until client exits!
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }

            println!("data send ok");
            TcpStream::shutdown(&mut socket).await.unwrap();
            println!("connect closed");
        });
    }
}
