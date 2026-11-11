use rxtx::StartNetSession;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    StartNetSession("127.0.0.1", 8686, |session| {
        println!("yo");
        session.transmit("Hello").await;
    }).await;
}
