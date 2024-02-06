#![cfg_attr(not(test), no_std)]

pub enum ControllerCommand {
    NameRequest = 0,   //Indicates the sensor's name.
    StatusRequest,     //For getting sensor modules status.
    ResetRequest,      //For asking the module to do a soft-reset.
    FormatingRequest,  //Gives the format of sensor's readings.
    DnamesRequest,     //Gives the data's names, (volts/temp/humidity etc)
    DataRequest,       //For requests of the sensor's data for individual type.
    BulkRequest,       //For requesting all the availble types of data.
}


//This gives the methods that must be implimented for any sensor that
//impliments the SensorInterface trait.
pub trait SensorInterface {
    //fn new() -> Box<dyn SensorInterface>;

    //fn init_sensor(&mut self) -> Result<Ok(()), SensorInterfaceError>;

    fn read_sensor(&mut self) -> &SensorData;

    fn get_format(&self) -> &'static str;

    fn get_name(&self) -> &'static str;

    fn get_names(&self) -> &'static str;

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
    fn read_sensor(&mut self) -> &SensorData {
        return &self.data; 
    }

    fn get_name(&self) -> &'static str {
        return self.sensor_name;
    }

    fn get_names(&self) -> &'static str {
        return READING_NAMES;
    }

    fn get_format(&self) -> &'static str {
        return READING_TYPES;
    }
}



#[cfg(test)]
mod sensor_interface_tests {
    use super::*;
    
    #[test]
    fn check_self() {
        assert!(true);
    }

    #[test]
    fn check_traits() {
        let sd = SensorData {
            data: [0x0F, 0xAA, 0x00, 0x55],
        }; 

        let exam = ExampleSensor {
            sensor_name: SENSOR_NAME,
            data_types: READING_TYPES,
            data_names: READING_NAMES,
            data: sd,
        };
        
        assert_eq!(exam.get_name(), SENSOR_NAME);
        assert_eq!(exam.get_format(), READING_TYPES);
        assert_eq!(exam.get_names(), READING_NAMES);

    }
}
