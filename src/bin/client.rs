use async_std::{net::TcpStream, task::sleep};
use std::{
    net::{Ipv6Addr, SocketAddrV6},
    time::Duration,
};

use json::object;
use rivertalk::packet;

#[async_std::main]
async fn main() {
    let address = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 2080, 0, 0);
    let mut connection: TcpStream = TcpStream::connect(address).await.unwrap();

    let obj = object! {
        message: "message!",
    };

    loop {
        packet::send(&mut connection, obj.clone()).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}
