use async_std::net::UdpSocket;
use futures::future::BoxFuture;

pub struct NetSession {
    sessionid: i32,
    targetport: i16,
    socket: UdpSocket,
    targethost: String,
}

impl NetSession {
    /// Transmit data of any type that implements Display.
    pub async fn transmit<T: std::fmt::Display>(&self, data: T) {
        println!("TX {}", data);
    }
}
pub async fn ListenNet(host: &str, port: i32, callback: impl Fn(i32, i32)) -> std::io::Result<()> {
    println!("Binding to {}:{}", host, port);

    let socket = UdpSocket::bind(format!("{}:{}", host, port)).await?;
    let mut buf = [0u8; 8192];

    loop {
        let (size, incoming_addr) = socket.recv_from(&mut buf).await?;

        println!("Recieved data from {} at size {}", incoming_addr, size);
    }
}

pub async fn StartNetSession<F>(host: &str, port: i32, func: F) -> std::io::Result<()>
    where F: AsyncFn(&NetSession)
{
    println!("{}:{} sending stuff", host, port);

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let session = NetSession {
        sessionid: 1i32,
        targetport: 8686i16,
        targethost: host.to_string(),
        socket: sock,
    };

    func(&session);

    return Ok(());
}
