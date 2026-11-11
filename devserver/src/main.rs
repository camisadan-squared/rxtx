use rxtx::ListenNet;
use tokio;

#[tokio::main]
async fn main() {
    ListenNet("192.168.1.1", 1000, |ipaddr: i32, packetsize: i32| {
        println!("{} with {}", ipaddr, packetsize);
    }).await;
}
