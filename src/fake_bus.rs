/*
 * Author: Jake G
 * Date: 2024
 * Filename: fake_bus.rs
 * Description: A fake implimentation of a bus for testing.
 */

const BUFFER_SIZE: usize = 32;
const MIN_ID: u32 =  0;
const MAX_ID: u32 = 128;
const LITTLE_ENDIAN: bool = true;
const BYTES_IN_U32: usize = 4;

#[allow(dead_code)]
pub struct FakeBus {
    tx_id: u32,
    rx_id: u32,
    num_bytes: usize,
    msg_buffer: [u8; BUFFER_SIZE],
}

impl FakeBus {
    pub fn new() -> FakeBus {
        let fb = FakeBus{
            tx_id: 0,
            rx_id: 1,
            num_bytes: 0,
            msg_buffer: [0; BUFFER_SIZE],
        };
        return fb;
    }

    //Returns the data bytes from the message.
    pub fn spy_data(&self) -> [u8; 8] {
        let mut spy_data: [u8; 8] = [0; 8];
        spy_data.copy_from_slice(&self.msg_buffer[4..(8 + 4)]);
        return spy_data;
    }

    //Returns the id of the message in the buffer.
    pub fn spy_id(&self) -> u32 {
        let id: u32;
        id = ((self.msg_buffer[0] as u32) | 
              ((self.msg_buffer[1] as u32)>>8) | 
              ((self.msg_buffer[2] as u32)>>16) | 
              ((self.msg_buffer[3] as u32)>>24)) as u32;
        return id;
    }
}

impl Bus for FakeBus {
    
    fn send_message(&mut self, id: u32, data: &[u8; SEND_BUFFER_BYTES], num_bytes: usize) -> Result<(), BusError> {
        if id > MAX_ID || id < MIN_ID || num_bytes > 8 || num_bytes < 1 {
            return Err(BusError::BadParameter);
        }
        
        //set the number of bytes used.
        self.num_bytes = num_bytes;

        //copy the id + data into the message_buffer, we do some bit shifting.
        let id_buf: [u8; BYTES_IN_U32];

        if LITTLE_ENDIAN {
            id_buf = id.to_le_bytes();  
        }
        else {
            id_buf = id.to_be_bytes();
        }
        self.msg_buffer[0..BYTES_IN_U32].copy_from_slice(&id_buf);
        
        //Now copy the data into the msg_buffer as well.
        self.msg_buffer[BYTES_IN_U32..(num_bytes + BYTES_IN_U32)].copy_from_slice(&data[0..num_bytes]);

        Ok(())
    }

    fn receive_message(&mut self) -> Result<(u32, [u8; READ_BUFFER_BYTES]), BusError> {
        let id: u32;
        let mut data: [u8; READ_BUFFER_BYTES] = [0; READ_BUFFER_BYTES];
        
        //Read the id from the message.
        if LITTLE_ENDIAN {
            id = ((self.msg_buffer[0] as u32) | 
                  ((self.msg_buffer[1] as u32)>>8) | 
                  ((self.msg_buffer[2] as u32)>>16) | 
                  ((self.msg_buffer[3] as u32)>>24)) as u32; 
        }
        else {
            id = ((self.msg_buffer[3] as u32) | 
                  ((self.msg_buffer[2] as u32)>>8) | 
                  ((self.msg_buffer[1] as u32)>>16) | 
                  ((self.msg_buffer[0] as u32)>>24)) as u32; 
        }

        //copy the message into the data array.
        data.copy_from_slice(
            &self
            .msg_buffer[
                BYTES_IN_U32..(READ_BUFFER_BYTES + BYTES_IN_U32)
                ]
            );

        Ok((id, data))
    }
}


#[cfg(test)]
mod fake_bus_tests {
    #[allow(unused_imports)]
    use super::*;

    //let mut fb = FakeBus::new();
    //let mut msg_data: [u8; 8] = [0; 8];

    #[macro_export]
    macro_rules! setup {
        ($($x:expr), *) => {
            let mut fb = FakeBus::new();
            let msg_data: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        };
    }

    #[test]
    fn check_self() {
        assert!(true);
    }

    #[test]
    fn send_receive() {
        let mut fb = FakeBus::new();
        let msg_data: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        assert!(fb.send_message(fb.rx_id, &msg_data, 8).is_ok());
        
        let result = fb.receive_message();
        assert!(result.is_ok());
        
        let rx_id: u32;
        let data: [u8; 8]; 
        (rx_id, data) = result.unwrap();

        assert!(rx_id == 1);
        assert!(data == msg_data);
    }

    #[test]
    fn send_receivce_single_byte() {
        let mut fb = FakeBus::new();
        let mut msg_data: [u8; 8] = [0; 8];
        
        //set the actual data into it
        msg_data[0] = 1;
        msg_data[1] = 6;

        //indicate we only want to read 1 byte
        assert!(fb.send_message(fb.rx_id, &msg_data, 1).is_ok());
        assert!(fb.num_bytes == 1);
        
        let result = fb.receive_message();
        assert!(result.is_ok());
        
        let rx_id: u32;
        let data: [u8; 8]; 
        (rx_id, data) = result.unwrap();

        assert!(rx_id == 1);
        msg_data = [0; 8];
        msg_data[0]= 1;
        assert!(data == msg_data);
    }

    #[test]
    fn send_bad_msg_len() {
        let mut fb = FakeBus::new();
        let mut msg_data: [u8; 8] = [0; 8];
        
        //set the actual data into it
        msg_data[0] = 1;
        msg_data[1] = 6;

        //indicate we only want to read 1 byte
        assert!(fb.send_message(fb.rx_id, &msg_data, 9).is_ok() == false);
    }

    #[test]
    fn send_bad_msg_id() {
        const INVALID_ID: u32 = 129;
        let mut fb = FakeBus::new();
        let mut msg_data: [u8; 8] = [0; 8];
        
        //set the actual data into it
        msg_data[0] = 1;
        msg_data[1] = 6;

        //indicate we only want to read 1 byte
        assert!(fb.send_message(INVALID_ID, &msg_data, 2).is_ok() == false);
    }

    #[test]
    fn spy_data() {
        let mut fb = FakeBus::new();
        let mut msg_data: [u8; 8] = [0; 8];

        //set the actual data into it
        msg_data[0] = 1;

        //indicate we only want to read 1 byte
        assert!(fb.send_message(fb.rx_id, &msg_data, 1).is_ok());
       
        //check that we can spy on the sent data.
        let spy_data = fb.spy_data();
        println!("Spy_data: {:?}", spy_data);
        
        assert!(spy_data == msg_data);

    }

    #[test]
    fn spy_id() {
        let mut fb = FakeBus::new();
        let mut msg_data: [u8; 8] = [0; 8];

        //set the actual data into it
        msg_data[0] = 1;

        //indicate we only want to read 1 byte
        assert!(fb.send_message(fb.rx_id, &msg_data, 1).is_ok());
        assert!(fb.spy_id() == fb.rx_id);
    }

    #[test]
    fn num_bytes() {

    }
}
