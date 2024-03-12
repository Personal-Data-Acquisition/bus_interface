//#![cfg_attr(test)]
//#![no_std]

mod cmd_return;
use cmd_return::CmdReturn;

/* Only include the fake/mocked when testing. */
#[cfg(test)]
include!("fake_bus.rs");


const _MAX_NAME_BYTES_LEN: usize = 64;
const _MAX_WAIT_MS: u32 = 500;
const SEND_BUFFER_BYTES: usize = 8;
const _READ_BUFFER_BYTES: usize = 8;
const CRONTROLLER_ID: u32 = 0;
const _CONTROLLER_BUFFER: usize = 256;



// The Errors that we allow as result's
#[derive(Debug)]
pub enum BusError {
    Unknown,
    BadParameter,
    BusError,
}

//A simplified bus setup. Will define wrappers for a variety of busses 
//elsewhere.
pub trait Bus{
    fn send_message(&mut self, id: u32, data: &Vec<u8>) -> Result<(), BusError>;
    fn receive_message(&mut self) -> Result<(u32, Vec<u8>), BusError>;
}


#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ControllerCommand {
    NameRequest = 0,   //Indicates the sensor's name.
    StatusRequest,     //For getting sensor modules status
    ResetRequest,      //For asking the module to do a soft-reset.
    FormatingRequest,  //Gives the format of sensor's readings.
    DnamesRequest,     //Gives the data's names, (volts/temp/humidity etc)
    DataRequest,       //For requests of the sensor's data for individual type.
    BulkRequest,       //For requesting all the availble types of data.
    BadCommand,        //To represent invalid or bad commands.
}

impl From<u8> for ControllerCommand {
    fn from(value: u8) -> Self {
        match value {
            0 => ControllerCommand::NameRequest,
            1 => ControllerCommand::StatusRequest,
            2 => ControllerCommand::ResetRequest,
            3 => ControllerCommand::FormatingRequest,
            4 => ControllerCommand::DnamesRequest,
            5 => ControllerCommand::DataRequest,
            6 => ControllerCommand::BulkRequest,
            _ => ControllerCommand::BadCommand,
        }
    }
}


// Used to indicate the various kinds of sensor module statuses/states.
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SensorStatus {
    Ready = 0,
    Busy,
    SensorFailure,
    PowerFailure,
    BusFailure,
    TempertureWarning,
    VoltageWarning,
}


//This gives the methods that must be implimented for any sensor that
//impliments the SensorInterface trait.
pub trait SensorInterface {

    fn get_name(&self) -> &'static str;
    
    fn get_status(&self) -> SensorStatus;

    fn soft_reset(&mut self) -> SensorStatus;

    fn get_format(&self) -> &'static str;

    fn get_data_names(&self) -> &'static str;

    fn read_sensor(&mut self) -> &SensorData;

}


#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusStatus {
    Good = 0,
    Busy,
    Error,
}


// Used by the BUS Master/Controller
pub fn send_bus_command(bus: &mut dyn Bus, cmd: &ControllerCommand) -> Result<CmdReturn, BusStatus>{
    
    let ret = CmdReturn::new();
    let mut data: Vec<u8> = Vec::with_capacity(SEND_BUFFER_BYTES);

    match cmd {
        ControllerCommand::NameRequest => {
            data.push(ControllerCommand::NameRequest as u8);
            let result = bus.send_message(CRONTROLLER_ID, &data);
            if result.is_ok() {
                return Ok(ret);
            }
            //impliment a timeout
            return Err(BusStatus::Error);
        }
        ControllerCommand::StatusRequest => {
            data.push(ControllerCommand::StatusRequest as u8);
            let result = bus.send_message(CRONTROLLER_ID, &data);
            if result.is_ok() {
                return Ok(ret);
            }
            return Err(BusStatus::Error);
        }
        ControllerCommand::ResetRequest => {
            data.push(ControllerCommand::ResetRequest as u8);
            let result = bus.send_message(CRONTROLLER_ID, &data);
            if result.is_ok() {
                return Ok(ret);
            }
            return Err(BusStatus::Error);
        }
        ControllerCommand::FormatingRequest => {
            data.push(ControllerCommand::FormatingRequest as u8);
            let result = bus.send_message(CRONTROLLER_ID, &data);
            if result.is_ok() {
                return Ok(ret);
            }
            return Err(BusStatus::Error);
        }
        ControllerCommand::DnamesRequest => {
            data.push(ControllerCommand::DnamesRequest as u8);
            let result = bus.send_message(CRONTROLLER_ID, &data);
            if result.is_ok() {
                return Ok(ret);
            }
            return Err(BusStatus::Error);
        }
        ControllerCommand::DataRequest => {
            Ok(ret)
        }
        ControllerCommand::BulkRequest => {
            Ok(ret)
        }
        ControllerCommand::BadCommand => {
            Ok(ret)
        }
    }
}

//Used by the slave device.
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
        ControllerCommand::BulkRequest => {

        }
        ControllerCommand::DataRequest => {

        }
        ControllerCommand::BadCommand => {

        }
    }

    //After processing then send the data.
    //Bus::send_message(

    Ok(()) 
}

#[allow(dead_code)]
pub struct SensorData {
    data: [u8; MAX_DATA],
}

//This is a structure just used to show how it works,
//you can think of this as a fake sensor; or an example of what you
//will need to create for the sensor interface.
#[allow(dead_code)]
struct ExampleSensor{
    sensor_name: &'static str,
    data_types: &'static str,
    data_names: &'static str,
    data: SensorData,
}


/*
 * This section shows how you should impliment
 * the traits for an sensor kinda.
 * The values for it are static strings that are comma seperated. 
 */
pub const MAX_DATA: usize = 4;
pub const SENSOR_NAME: &str = "Fakesensor";
pub const READING_NAMES: &str = "Temperature, Humidity";
pub const READING_TYPES: &str = "u16, u16";
impl SensorInterface for ExampleSensor {

    fn get_name(&self) -> &'static str {
        return self.sensor_name;
    }

    fn get_status(&self) -> SensorStatus {
        return SensorStatus::Ready;
    }

    fn soft_reset(&mut self) -> SensorStatus {
        return SensorStatus::Busy;
    }

    fn get_format(&self) -> &'static str {
        return READING_TYPES;
    }

    fn get_data_names(&self) -> &'static str {
        return READING_NAMES;
    }

    fn read_sensor(&mut self) -> &SensorData {
        return &self.data; 
    }

}


#[cfg(test)]
mod sensor_interface_tests {
    use super::*;

    struct TestData{
        sens: ExampleSensor,
        bus: FakeBus,
    }

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
        
        let td = TestData{
            sens: fake_sensor,
            bus: fake_bus,
        };
        
        td
    }

    #[test]
    fn check_self() {
        assert!(true);
    }

    #[test]
    fn check_traits() {
        let sd = SensorData {
            data: [0x0F, 0xAA, 0x00, 0x55],
        }; 

        let mut exam = ExampleSensor {
            sensor_name: SENSOR_NAME,
            data_types: READING_TYPES,
            data_names: READING_NAMES,
            data: sd,
        };
        
        assert_eq!(exam.get_name(), SENSOR_NAME);
        assert_eq!(exam.get_format(), READING_TYPES);
        assert_eq!(exam.get_data_names(), READING_NAMES);
        assert_eq!(exam.soft_reset(), SensorStatus::Busy);
        assert_eq!(exam.get_status(), SensorStatus::Ready);
    }
   
    #[test]
    fn into_and_from() {
        let val: u8  = 0x00;
        assert_eq!(ControllerCommand::from(val), ControllerCommand::NameRequest);
    }

    #[test]
    fn read_name_command() {
        let id: u32 = 0x001; 
        let mut td = setup();

        /* SERVER SIDE ACTIONS */
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::NameRequest);
        assert!(cmd_result.is_ok());

        //now check the send data.
        println!("data: {:?}", td.sens.data.data);
        println!("bus data: {:?}", td.bus.spy_data());
        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::NameRequest as u8);

        /* CLIENT SIDE ACTIONS */        
        let handler_result = handle_bus_command(id, &mut td.bus, &mut td.sens);
        assert!(handler_result.is_ok());

        //check that the data is sent back.
        assert_eq!(td.bus.spy_data(), td.sens.sensor_name.as_bytes());

    }

    #[test]
    fn status_request() {
        let mut td = setup();
        
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::StatusRequest);
        assert!(cmd_result.is_ok());

        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::StatusRequest as u8);
       
        //check the received sensor status.
        let handler_result = handle_bus_command(0x001, &mut td.bus, &mut td.sens);
        assert!(handler_result.is_ok());

        //check the status was sent back.
        assert_eq!(td.bus.spy_data()[0], td.sens.get_status() as u8);
    }

    #[test]
    fn reset_request() {
        let mut td = setup();
        
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::ResetRequest);
        assert!(cmd_result.is_ok());

        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::ResetRequest as u8);

        let handler_result = handle_bus_command(0x001, &mut td.bus, &mut td.sens);
        assert!(handler_result.is_ok());

        //check the status was sent back.
        assert_eq!(td.bus.spy_data()[0], td.sens.soft_reset() as u8);
    }

    #[test]
    fn formatting_request() {
        
        let mut td = setup();
        
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::FormatingRequest);
        assert!(cmd_result.is_ok());

        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::FormatingRequest as u8);

        let handler_result = handle_bus_command(0x001, &mut td.bus, &mut td.sens);
        assert!(handler_result.is_ok());

        //check the formatting sent back.
        assert_eq!(READING_TYPES.as_bytes() , td.bus.spy_data());
    }

    #[test]
    fn data_names_request() {

        let mut td = setup();
        
        let cmd_result = send_bus_command(&mut td.bus, &ControllerCommand::DnamesRequest);
        assert!(cmd_result.is_ok());

        assert!(td.bus.spy_id() == 0);
        assert!(td.bus.spy_data()[0] == ControllerCommand::DnamesRequest as u8);

        let handler_result = handle_bus_command(0x001, &mut td.bus, &mut td.sens);
        assert!(handler_result.is_ok());

        //check the formatting sent back.
        assert_eq!(READING_NAMES.as_bytes() , td.bus.spy_data());
    }
}
