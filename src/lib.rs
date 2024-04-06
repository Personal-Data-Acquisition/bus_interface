//Support using without the standard library
#![cfg_attr(all(not(feature = "bus_master"), not(test)), no_std)]

/* Use an allocator if we aren't in a std enviroment or testing.*/
#[cfg(any(not(test), feature = "sensor_module"))]
extern crate alloc;

/* Include the `Vec` type from alloc */
#[cfg(any(not(test), feature = "sensor_module"))]
use alloc::vec::Vec;

/* Use the `vec` macro from alloc */
#[cfg(any(not(test), feature = "sensor_module"))]
use alloc::vec;


#[cfg(any(not(test), feature = "sensor_module"))]
use core::include;


const _MAX_NAME_BYTES_LEN: usize = 64;
const _MAX_WAIT_MS: u32 = 500;
const _SEND_BUFFER_BYTES: usize = 8;
const _READ_BUFFER_BYTES: usize = 8;
#[allow(dead_code)]
const CRONTROLLER_ID: u32 = 0; /*Not actually dead code, called by controller*/
const _CONTROLLER_BUFFER: usize = 256;
const MAX_DATA: usize = 4;


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
    FormattingRequest,  //Gives the format of sensor's readings.
    DnamesRequest,     //Gives the data's names, (volts/temp/humidity etc)
    DataRequest,       //For requests of the sensor's data for individual type.
}

impl From<u8> for ControllerCommand {
    fn from(value: u8) -> Self {
        match value {
            0 => ControllerCommand::NameRequest,
            1 => ControllerCommand::StatusRequest,
            2 => ControllerCommand::ResetRequest,
            3 => ControllerCommand::FormattingRequest,
            4 => ControllerCommand::DnamesRequest,
            5 => ControllerCommand::DataRequest,
            _ => ControllerCommand::ResetRequest
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

    fn read_sensor(&mut self, idx: u8) -> &SensorData;

}


#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusStatus {
    Good = 0,
    Busy,
    Error,
    DataErr,
}

#[allow(dead_code)]
pub struct SensorData {
    data: [u8; MAX_DATA],
    size: usize,
}



/* Include all the files when we test them. */

#[cfg(test)]
include!("fake_sensor.rs");

#[cfg(test)]
include!("fake_bus.rs");

#[cfg(any(test, feature = "bus_master"))]
include!("controller.rs");

#[cfg(any(test, feature = "sensor_module"))]
include!("handler.rs");


