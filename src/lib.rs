pub mod packet;

use async_std::net::{TcpListener, TcpStream};
use futures::{select, stream::FuturesUnordered, StreamExt};
use json::JsonValue;
use std::net::SocketAddrV6;

pub struct Error;

pub struct Client {
    hash: [u8; 512],
    stream: TcpStream,
}

pub struct Server {
    clients: Vec<Client>,
    address: SocketAddrV6,
}

impl Server {
    pub fn new(address: SocketAddrV6) -> Self {
        Self {
            clients: vec![],
            address,
        }
    }

    pub async fn run(self: Self) -> Self {
        self
    }

    async fn receive(mut self: Self) -> JsonValue {
        let mut incoming = FuturesUnordered::new();
        self.clients
            .iter_mut()
            .for_each(|client| incoming.push(packet::receive(&mut client.stream)));

        select! {
            here = incoming.select_next_some() => return here.unwrap()
        }
    }

    async fn new_client(self: Self) -> TcpStream {
        TcpListener::bind(self.address)
            .await
            .unwrap()
            .incoming()
            .next()
            .await
            .unwrap()
            .unwrap()
    }

    // TODO! refactor this to actually use concurrency
    async fn distribute(mut self: Self, obj: JsonValue) -> Result<(), packet::Error> {
        let mut outgoing = FuturesUnordered::new();
        self.clients
            .iter_mut()
            .for_each(|client| outgoing.push(packet::send(&mut client.stream, obj.clone())));

        // TODO! this doesn't actually send out the packets just yet

        Ok(())
    }
}
