use async_std::net::TcpStream;
use json::JsonValue;
use std::net::SocketAddrV6;

use crate::packet;

#[derive(Debug)]
pub struct Error(&'static str);

struct Server {
    stream: TcpStream,
}

impl Server {
    async fn new(address: SocketAddrV6) -> Result<Self, Error> {
        match TcpStream::connect(address).await {
            Ok(stream) => Ok(Self { stream }),
            Err(_) => Err(Error("Cannot connect to server")),
        }
    }
}

pub struct Client {
    server: Server,
}

impl Client {
    pub async fn new(address: SocketAddrV6) -> Result<Self, Error> {
        Ok(Self {
            server: Server::new(address).await?,
        })
    }

    pub async fn run(mut self: Self) -> Result<Self, Error> {
        unimplemented!();
    }

    async fn receive(mut server: Server) -> Result<JsonValue, packet::Error> {
        packet::receive(&mut server.stream).await
    }

    async fn send(mut server: Server, obj: JsonValue) -> Result<(), packet::Error> {
        packet::send(&mut server.stream, obj).await
    }
}
