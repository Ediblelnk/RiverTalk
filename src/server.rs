use async_std::net::{TcpListener, TcpStream};
use futures::{pin_mut, select, stream::FuturesUnordered, FutureExt, StreamExt};
use json::JsonValue;
use std::net::SocketAddrV6;

use crate::packet;

#[derive(Debug)]
pub struct Error;

#[derive(Clone)]
pub struct Client {
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

    pub async fn run(mut self: Self) -> Result<Self, Error> {
        let first_client = Self::new_client(self.address).await;
        self.clients.push(Client {
            stream: first_client,
        });

        loop {
            let new_client = Self::new_client(self.address).fuse();
            let receive = Self::receive(self.clients.clone()).fuse();

            pin_mut!(new_client, receive);

            select! {
                stream = new_client => self.clients.push(Client { stream }),
                obj = receive => println!("{obj}")
            }
        }
        Ok(self)
    }

    async fn receive(mut clients: Vec<Client>) -> JsonValue {
        let mut incoming = FuturesUnordered::new();
        clients
            .iter_mut()
            .for_each(|client| incoming.push(packet::receive(&mut client.stream)));

        select! {
            here = incoming.select_next_some() => return here.unwrap()
        }
    }

    async fn new_client(address: SocketAddrV6) -> TcpStream {
        TcpListener::bind(address)
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

        while !outgoing.is_empty() {
            select! {
                gone = outgoing.select_next_some() => gone.unwrap()
            }
        }

        Ok(())
    }
}
