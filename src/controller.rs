/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: controller.rs
 * Desc: File to be included for SBC(single board computer). 
 */


// Used by the BUS Master/Controller
pub fn send_bus_command(bus: &mut dyn Bus, cmd: &ControllerCommand) -> Result<CmdReturn, BusStatus>{
    
    let mut ret = CmdReturn::new();
    let mut data: Vec<u8> = vec![]; // Vec::with_capacity(SEND_BUFFER_BYTES);

    match cmd {
        ControllerCommand::NameRequest => {
            data.push(ControllerCommand::NameRequest as u8);
        }
        ControllerCommand::StatusRequest => {
            data.push(ControllerCommand::StatusRequest as u8);
        }
        ControllerCommand::ResetRequest => {
            data.push(ControllerCommand::ResetRequest as u8);
        }
        ControllerCommand::FormatingRequest => {
            data.push(ControllerCommand::FormatingRequest as u8);
        }
        ControllerCommand::DnamesRequest => {
            data.push(ControllerCommand::DnamesRequest as u8);
        }
        ControllerCommand::DataRequest => {
            data.push(ControllerCommand::DnamesRequest as u8);
        }
        ControllerCommand::BulkRequest => {
            data.push(ControllerCommand::BulkRequest as u8);
        }
        ControllerCommand::BadCommand => {
            data.push(ControllerCommand::BadCommand as u8);
        }
    }
    
    let result = bus.send_message(CRONTROLLER_ID, &data);
    if result.is_err() {
        return Err(BusStatus::Error);
    }

    
    /* Now we try to get the response from the bus */
    let result = bus.receive_message();
    if result.is_err() {
        return Err(BusStatus::Error);
    }
    
    let _id: u32;
    let data: Vec<u8>;
    (_id, data) = result.ok().unwrap();

    println!("cmd: {:?}", cmd);

    match cmd {
        ControllerCommand::NameRequest => {
            let str_encode_ret = String::from_utf8(data);
            match str_encode_ret {
                Ok(v) => ret.name = v,
                Err(_e) => return Err(BusStatus::DataErr),
            };
        }
        ControllerCommand::StatusRequest => {
            ret.data_names.push(String::from("Status"));
            ret.format.push(String::from("u8"));
            ret.raw_bytes.push(data[0]);
        }
        ControllerCommand::ResetRequest => {
            ret.data_names.push(String::from("Status"));
            ret.format.push(String::from("u8"));
            ret.raw_bytes.push(data[0]);
        }
        ControllerCommand::FormatingRequest => {
        }
        ControllerCommand::DnamesRequest => {
        }
        ControllerCommand::DataRequest => {
        }
        ControllerCommand::BulkRequest => {
        }
        ControllerCommand::BadCommand => {
        }
    }
    println!("ret: {:?}", ret);
    return Ok(ret);
}

#[allow(dead_code)]
fn initiate_request(cmd: ControllerCommand) {
    assert!(false);
}


#[cfg(test)]
mod controller_tests {
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
    fn name_request() {
        let mut td = setup();

        // preload the response into the msg buffer.
        let name_data: Vec<u8> = SENSOR_NAME.as_bytes().to_vec();
        let set_res = td.bus.set_rmsg_data(&name_data);
        assert!(set_res.is_ok());

        // send the controller command
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::NameRequest);
        assert!(cmd_result.is_ok());

        // now check the send data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::NameRequest as u8);

        // chcek the actual returned value.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(SENSOR_NAME, cmd_data.name);
    }

    #[test]
    fn status_request() {
        let mut td = setup();

        // Preload the response
        let status_data: Vec<u8> = vec![SensorStatus::Ready as u8]; 
        assert!(td.bus.set_rmsg_data(&status_data).is_ok());
       
        // send the controller command
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::StatusRequest);
        assert!(cmd_result.is_ok());

        // now check the send data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::StatusRequest as u8);
        
        // Check returned data.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(cmd_data.format[0], "u8");
        assert_eq!(cmd_data.data_names[0], "Status");
        assert_eq!(SensorStatus::Ready as u8, cmd_data.raw_bytes[0]); 
    }

    #[test]
    fn reset_request() {
        let mut td = setup();

        // Preload the response
        let reset_data: Vec<u8> = vec![SensorStatus::Busy as u8]; 
        assert!(td.bus.set_rmsg_data(&reset_data).is_ok());

        // Send the controller cmd
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::ResetRequest);
        assert!(cmd_result.is_ok());

        // now check the send data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::ResetRequest as u8);

        // check returned data.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(cmd_data.format[0], "u8");
        assert_eq!(cmd_data.data_names[0], "Status");
        assert_eq!(SensorStatus::Busy as u8, cmd_data.raw_bytes[0]); 
    }
}
