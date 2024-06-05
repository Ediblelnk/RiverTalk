use json::JsonValue;

pub trait Receiver {
    fn receive(&self) -> JsonValue;
}

pub trait Distributor {
    fn distribute(&self, obj: JsonValue) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    Fatal(String),
}

pub struct Server {
    receiver: Box<dyn Receiver>,
    distributor: Box<dyn Distributor>, 
}

impl Server {
    pub fn new(receiver: Box<dyn Receiver>, distributor: Box<dyn Distributor>) -> Self {
        Self {receiver, distributor}
    }

    pub fn run(self: Self) -> Self {
        let obj = self.receiver.receive();
        self.distributor.distribute(obj).unwrap();
        self
    }
}
