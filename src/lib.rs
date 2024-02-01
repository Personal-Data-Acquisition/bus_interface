#![cfg_attr(not(test), no_std)]


#[allow(dead_code)]
struct SensorInterface {
    sensor_name: &'static str,
    data_types: &'static str,
    data_names: &'static str,
}


#[cfg(test)]
mod sensor_interface_tests {
    use super::*;
    
    #[test]
    fn check_self() {
        assert!(true);
    }

    #[test]
    fn init_sensor() {
        assert!(false);
    }

    #[test]
    fn read_sensor() {
        assert!(false);
    }

    #[test]
    fn get_format() {
        assert!(false);
    }

    #[test]
    fn get_names() {
        assert!(false);
    }
}


