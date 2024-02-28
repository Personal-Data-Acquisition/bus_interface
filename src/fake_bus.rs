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

#[allow(dead_code)]
pub struct FakeBus {
    id: u32,
    msg_buffer: [u8; BUFFER_SIZE],
}

impl Bus for FakeBus {
    
    fn send_message(&mut self, id: u32, data: &[u8; SEND_BUFFER_BYTES]) -> Result<(), BusError> {
        if id > MAX_ID || id < MIN_ID {
            return Err(BusError::Unknown);
        }
        
        //copy the id + data into the message_buffer, we do some bit shifting.
        let id_buf: [u8; 4];

        if LITTLE_ENDIAN {
            id_buf = id.to_le_bytes();  
        }
        else {
            id_buf = id.to_be_bytes();
        }
        self.msg_buffer.copy_from_slice(&id_buf);
        
        //Now copy the data into the msg_buffer as well.
        self.msg_buffer[4..].copy_from_slice(data);

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
        data.copy_from_slice(&self.msg_buffer);

        Ok((id, data))
    }
}
