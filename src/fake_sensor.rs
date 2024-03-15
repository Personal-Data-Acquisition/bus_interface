/*
 * Authors: Jake G,
 * Date: 2024
 * Filename: fake_sensor.rs
 * Desc: Impliments the fake sensor for testing. 
 */


pub const NUM_TYPES: usize = 3;

/*
 * This section shows how you should impliment
 * the traits for an sensor kinda.
 */
pub const SENSOR_NAME: &str = "Fakesensor";
pub const READING_NAMES: &str = "Status Temp Humid";
pub const READING_TYPES: &str = "u8 u16 u16";


//This is a structure just used to show how it works,
//you can think of this as a fake sensor; or an example of what you
//will need to create for the sensor interface.
#[allow(dead_code)]
struct ExampleSensor{
    sensor_name: &'static str,
    data_types: [&'static str; NUM_TYPES],
    data_names: [&'static str; NUM_TYPES],
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

    fn read_sensor(&mut self, idx: u8) -> &SensorData {
        // Read the fake sensor. 
        match self.data_types[idx as usize] {
            "u8" => self.data.size = 1,
            "i8" => self.data.size = 1,
            "u16" => self.data.size = 2,
            "i16" => self.data.size = 2,
            "u32" => self.data.size = 4,
            "i32" => self.data.size = 4,
            "f32" => self.data.size = 4,
            _ => self.data.size = 0,
        }
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
            size: 4
        }; 
        
        let fake_sensor = ExampleSensor {
                sensor_name: SENSOR_NAME,
                data_types: ["u8", "u16", "u16"],
                data_names: ["Status", "Temp", "Humid"],
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
    fn test_read_data() {
        let mut td = setup();

        td.sens.read_sensor(1);
        assert!(td.sens.data.size == 2);

        td.sens.read_sensor(0);
        assert!(td.sens.data.size == 1);

        td.sens.read_sensor(2);
        assert!(td.sens.data.size == 2);
    }
}
