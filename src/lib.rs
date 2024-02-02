#![cfg_attr(not(test), no_std)]

//This gives the methods that must be implimented for any sensor that
//impliments the SensorInterface trait.
pub trait SensorInterface {
    fn new() -> SensorInterface;

    fn init_sensor(&self) -> Result<T, E>;

    fn read_sensor(&self) -> SensorData;

    fn get_format(&self) -> &'static str;

    fn get_names(&self) -> &'static str;
}

#[allow(dead_code)]
struct SensorData {
    data: u8,
}

//This is a structure just used to show how it works,
//you can think of this as a fake sensor; or an example of what you
//will need to create for the sensor interface.
#[allow(dead_code)]
struct ExampleSensor{
    sensor_name: &'static str,
    data_types: &'static str,
    data_names: &'static str,
}

/*
 * This section shows how you should impliment
 * the traits for an sensor kinda.
 * 
 */
impl SensorInterface for ExampleSensor {

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
        let exam = ExampleSensor::new();
    }
}
