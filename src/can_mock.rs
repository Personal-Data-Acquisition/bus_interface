
// Define the CAN bus trait
pub trait CanBus {
    fn send_message(&mut self, id: u32, data: &[u8]) -> Result<(), String>;
    fn receive_message(&mut self) -> Result<(u32, Vec<u8>), String>;
}

// Define a mock CAN bus struct
pub struct MockCanBus {
    messages: Vec<(u32, Vec<u8>)>,
}

// Implement the CanBus trait for the mock struct
impl MockCanBus {
    pub fn new() -> MockCanBus {
        MockCanBus { messages: vec![] }
    }
}

impl CanBus for MockCanBus {
    fn send_message(&mut self, id: u32, data: &[u8]) -> Result<(), String> {
        self.messages.push((id, data.to_vec()));
        Ok(())
    }

    fn receive_message(&mut self) -> Result<(u32, Vec<u8>), String> {
        if let Some(msg) = self.messages.pop() {
            Ok(msg)
        } else {
            Err("No messages in buffer".to_string())
        }
    }
}

// Example usage
fn main() {
    let mut can_bus = MockCanBus::new();
    can_bus.send_message(1, &[0x11, 0x22, 0x33]).unwrap();
    let (id, data) = can_bus.receive_message().unwrap();
    println!("Received message: ID={}, Data={:X?}", id, data);
}

