use rivertalk::server::Server;
use std::net::{Ipv6Addr, SocketAddrV6};

#[async_std::main]
async fn main() {
    let address = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 2080, 0, 0);
    let server = Server::new(address);

    server.run().await.unwrap();
}
