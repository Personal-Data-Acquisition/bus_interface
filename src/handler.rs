/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: handler.rs
 * Desc: File to be included for embedded devices. 
 */

//include!("fake_sensor.rs");

pub fn handle_bus_command(slv_id: u32, bus: &mut dyn Bus, sens: &mut dyn SensorInterface) -> Result<(), BusError>{
    
    //get the cmd out of the message.
    let result = bus.receive_message()?;

    let id;
    let mut _master_data: Vec<u8> = vec![]; 
    (id, _master_data) = result;
    let cmd: ControllerCommand = _master_data[0].into();
    println!("id: {:?}\n", id);

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
        ControllerCommand::FormatingRequest => {
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
            data: [0x0F, 0xAA, 0x00, 0x55],
        }; 
        
        let fake_sensor = ExampleSensor {
                sensor_name: SENSOR_NAME,
                data_types: READING_TYPES,
                data_names: READING_NAMES,
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
    fn formating_handler() {
        
        let mut td = setup();
        let slv_id: u32 = 0x01;

        // Preload the needed test data.
        let data: Vec<u8> = vec![ControllerCommand::FormatingRequest as u8];
        assert!(td.bus.set_rmsg_data(&data).is_ok());

        // Call the code under test.
        assert!(handle_bus_command(slv_id, &mut td.bus, &mut td.sens).is_ok());
        
        // Check that the response is correct.
        assert_eq!(td.bus.spy_data(), td.sens.data_types.as_bytes());
    }
}
