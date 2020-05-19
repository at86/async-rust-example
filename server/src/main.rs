use futures::stream::StreamExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = {
        async move {
            let mut incoming = listener.incoming();
            // Using impl Stream:
            while let Some(conn) = incoming.next().await {
                sen::atlog!("incoming...");
                match conn {
                    Err(e) => eprintln!("accept failed = {:?}", e),
                    Ok(mut sock) => {
                        tokio::spawn(async move {
                            let (mut reader, mut writer) = sock.split();
                            tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;
                            match tokio::io::copy(&mut reader, &mut writer).await {
                                Ok(amt) => {
                                    sen::atlog!("wrote {} bytes", amt);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                        });
                    }
                }
            }
        }
    };
    println!("Server running on localhost:6142");
    server.await;
}
