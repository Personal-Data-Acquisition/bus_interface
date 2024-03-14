/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: controller.rs
 * Desc: File to be included for SBC(single board computer). 
 */


// Used by the BUS Master/Controller
pub fn send_bus_command(
    bus: &mut dyn Bus,
    cmd: &ControllerCommand,
    dname: String) -> Result<CmdReturn,BusStatus>
{
    
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
        ControllerCommand::FormattingRequest => {
            data.push(ControllerCommand::FormattingRequest as u8);
        }
        ControllerCommand::DnamesRequest => {
            data.push(ControllerCommand::DnamesRequest as u8);
        }
        ControllerCommand::DataRequest => {
            data.push(ControllerCommand::DataRequest as u8);
            for byte in dname.into_bytes().iter() {
                data.push(*byte);
            }
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
        ControllerCommand::FormattingRequest => {
            ret.raw_bytes = data;
            let res = ret.parse_raw_to_format();
            if ! res.is_ok() {
                return Err(BusStatus::DataErr);
            }
        }
        ControllerCommand::DnamesRequest => {
            ret.raw_bytes = data;
            let res = ret.parse_raw_to_dnames();
            if ! res.is_ok() {
                return Err(BusStatus::DataErr);
            }
        }
        ControllerCommand::DataRequest => {
            //just copy the raw_data over in this case.
            ret.raw_bytes = data;
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
    fn name_request() {
        let mut td = setup();

        // preload the response into the msg buffer.
        let name_data: Vec<u8> = SENSOR_NAME.as_bytes().to_vec();
        let set_res = td.bus.set_rmsg_data(&name_data);
        assert!(set_res.is_ok());

        // send the controller command
        let dname: String = String::new(); 
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::NameRequest, dname);
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
        let dname: String = String::new(); 
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::StatusRequest, dname);
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
        let dname: String = String::new(); 
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::ResetRequest, dname);
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

    #[test]
    fn formatting_request() {
        
        let mut td = setup();

        // Preload the response
        let format_data: Vec<u8> = String::from("u8 u16 u16").into_bytes(); 
        assert!(td.bus.set_rmsg_data(&format_data).is_ok());

        // Send the controller cmd
        let dname: String = String::new(); 
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::FormattingRequest, dname);
        assert!(cmd_result.is_ok());

        // now check the send data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::FormattingRequest as u8);

        // check returned data.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(cmd_data.format[0], "u8");
        assert_eq!(cmd_data.format[1], "u16");
        assert_eq!(cmd_data.format[2], "u16");
    }

    #[test]
    fn data_names_request() {
        let mut td = setup();

        // Preload the response.
        let data_names: Vec<u8> = String::from("Status Temp Humid").into_bytes();
        assert!(td.bus.set_rmsg_data(&data_names).is_ok());

        // Send the controller cmd
        let dname: String = String::new(); 
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::DnamesRequest, dname);
        assert!(cmd_result.is_ok());

        // Now check the sent data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::DnamesRequest as u8);

        // Check the returned data.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(cmd_data.data_names[0], "Status");
        assert_eq!(cmd_data.data_names[1], "Temp");
        assert_eq!(cmd_data.data_names[2], "Humid");
    }


    #[test]
    fn data_request() {
        let mut td = setup();

        // Preload the response.
        //let sensor_data: Vec<u8> = vec![0, 0, 255, 0, 255];
        let sensor_data: Vec<u8> = vec![0, 255];
        assert!(td.bus.set_rmsg_data(&sensor_data).is_ok());

        // Send the controller cmd
        let dname: String = String::from("Temp");
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::DataRequest, dname);
        assert!(cmd_result.is_ok());

        // Now check the sent data.
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::DataRequest as u8);

        // Check the returned data.
        let cmd_data = cmd_result.ok().unwrap();
        assert_eq!(cmd_data.raw_bytes[0], sensor_data[0]);
        assert_eq!(cmd_data.raw_bytes[1], sensor_data[1]);
    }
}
