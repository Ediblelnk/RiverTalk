use json::JsonValue;

fn main() {
    
}

trait Receiver {
    fn receive(&self) -> JsonValue;
}

trait Distributor {
    fn distribute(&self) -> Result<(), RiverTalkError>;
}

enum RiverTalkError {
    Fatal(String),
}

struct RiverTalk<'a> {
    receiver: &'a dyn Receiver,
    distributor: dyn Distributor 
}