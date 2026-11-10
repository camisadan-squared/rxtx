use async_std::net::UdpSocket;


pub async fn ListenNet(host: &str,port: i32 ,callback: impl Fn(i32,i32,)) -> std::io::Result<()> {

    print!("Binding to {}:{}",host,port);

    let socket = UdpSocket::bind(format!("{}:{}",host,port)).await?;
    let mut buf = [0u8; 8192];

    loop {
        let (size,incoming_addr) = socket.recv_from(&mut buf).await?;
    
        print!("Recieved data from {} at size {}",incoming_addr,size)
    }

}