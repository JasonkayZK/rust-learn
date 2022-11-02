use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut f = File::open("foo.txt").await.unwrap();
    let mut buffer = [0; 1000];

    // read up to 1000 bytes
    while let Ok(cnt) = f.read(&mut buffer).await {
        if cnt == 0 {
            break;
        }
        println!("The bytes: {:?}", &buffer[..cnt]);
    }
}
