/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: handler.rs
 * Desc: File to be included for embedded devices. 
 */

/* Use an allocator if we aren't in a std enviroment or testing.*/
#[cfg(any(not(test), feature = "sensor_module"))]
extern crate alloc;

/* Include the `Vec` type from alloc */
#[cfg(any(not(test), feature = "sensor_module"))]
use alloc::vec::Vec;

/* Use the `vec` macro from alloc */
#[cfg(any(not(test), feature = "sensor_module"))]
use alloc::vec;


pub fn handle_bus_command(slv_id: u32, bus: &mut dyn Bus, sens: &mut dyn SensorInterface) -> Result<(), BusError>{
    
    //get the cmd out of the message.
    let result = bus.receive_message()?;

    let _id;
    let master_data: Vec<u8>;
    (_id, master_data) = result;
    let cmd: ControllerCommand = master_data[0].into();

    let mut write_buf: Vec<u8> = vec![];
    
    //match the command so we can call a handler.
    match cmd {
        ControllerCommand::NameRequest => {
            //get the data from the sensor interface.
            let name = sens.get_name().as_bytes();            
            
            for i in 0..name.len() {
                write_buf.push(name[i]);
            }

            //send the data.              
            bus.send_message(slv_id, &write_buf)?;

        }
        ControllerCommand::StatusRequest => {
            let status = sens.get_status() as u8;
            write_buf.push(status); 
            bus.send_message(slv_id, &write_buf)?;

        }
        ControllerCommand::ResetRequest => {
            let status = sens.soft_reset() as u8;
            write_buf.push(status); 
            bus.send_message(slv_id, &write_buf)?;

        }
        ControllerCommand::FormattingRequest => {
            let formatting = sens.get_format().as_bytes(); 
            
            for i in 0..formatting.len() {
                write_buf.push(formatting[i]);
            }

            bus.send_message(slv_id, &write_buf)?;

        }
        ControllerCommand::DnamesRequest => {

            let data_names = sens.get_data_names().as_bytes(); 
            
            for i in 0..data_names.len() {
                write_buf.push(data_names[i]);
            }

            bus.send_message(slv_id, &write_buf)?;

        }
        ControllerCommand::DataRequest => {
            // The '1' index of the sent data indicates the sensor info 
            // that is being requested.
            let data_index = master_data[1];

            // The sensor info returned is based off the index.
            let sensor_info = sens.read_sensor(data_index);
            for i in 0..sensor_info.size {
                write_buf.push(sensor_info.data[i]);
            }

            bus.send_message(slv_id, &write_buf)?;
        }
    }


    Ok(()) 
}


#[cfg(test)]
mod handler_tests {
    use super::*;

    #[allow(dead_code)]
    struct TestData{
        sens: ExampleSensor,
        bus: FakeBus,
    }

    #[allow(dead_code)]
    fn setup() -> TestData {
        let sd = SensorData {
            data: [0xAA, 0x55, 0x00, 0x55],
            size: 4,
        }; 
        
        let fake_sensor = ExampleSensor {
                sensor_name: SENSOR_NAME,
                data_types: ["u8", "u16", "u16"],
                data_names: ["Status", "Temp", "Humid"],
                data: sd,
        };

        let fake_bus = FakeBus::new();
        
        let mut td = TestData{
            sens: fake_sensor,
            bus: fake_bus,
        };
        td.bus.auto_response = true;
        td
    }

    #[test]
    fn check_self() {
        assert!(true);
    }

    #[test]
    fn name_handler() {
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::NameRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        assert_eq!(td.bus.spy_data(), td.sens.sensor_name.as_bytes());
    }


    #[test]
    fn status_handler() {
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::StatusRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        assert_eq!(td.bus.spy_data()[0], td.sens.get_status() as u8);
    }

    #[test]
    fn reset_handler() {
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::ResetRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        assert_eq!(td.bus.spy_data()[0], td.sens.soft_reset() as u8);
    }

    #[test]
    fn formatting_handler() {
        
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::FormattingRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        let mut tmps: String = String::new();
        tmps.push_str(td.sens.data_types[0]); tmps.push_str(" ");
        tmps.push_str(td.sens.data_types[1]); tmps.push_str(" ");
        tmps.push_str(td.sens.data_types[2]); 
        assert_eq!(td.bus.spy_data(), tmps.into_bytes());
    }

    #[test]
    fn dnames_handler() {
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::DnamesRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        let mut tmps: String = String::new();
        tmps.push_str(td.sens.data_names[0]); tmps.push_str(" ");
        tmps.push_str(td.sens.data_names[1]); tmps.push_str(" ");
        tmps.push_str(td.sens.data_names[2]);
        assert_eq!(td.bus.spy_data(), tmps.into_bytes());
    }

    #[test]
    fn data_handler() {
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data_index: u8 = 1; //Should equate to "temp" 
        let mut data: Vec<u8> = vec![ControllerCommand::DataRequest as u8];
        data.push(data_index);
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        println!("spy_data {:?}", td.bus.spy_data());
        println!("sensor data size: {}", td.sens.data.size);
        println!("sensor_data: {:?}", td.sens.data.data);
        assert_eq!(td.bus.spy_data()[0], td.sens.data.data[0]);
        assert_eq!(td.bus.spy_data()[1], td.sens.data.data[1]);
    }

}
