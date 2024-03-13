/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: fake_sensor.rs
 * Desc: Impliments the fake sensor for testing. 
 */


/*
 * This section shows how you should impliment
 * the traits for an sensor kinda.
 * The values for it are static strings that are comma seperated. 
 */
pub const SENSOR_NAME: &str = "Fakesensor";
pub const READING_NAMES: &str = "Temperature, Humidity";
pub const READING_TYPES: &str = "u16, u16";


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
mod fake_sensor_test {
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
}
