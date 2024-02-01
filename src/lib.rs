#![cfg_attr(not(test), no_std)]



#[allow(dead_code)]
struct SensorData {
    data: u8,
}

#[allow(dead_code)]
struct SensorInterface {
    sensor_name: &'static str,
    data_types: &'static str,
    data_names: &'static str,
}

pub trait InitSensor {
    fn init_sensor(&self) -> Result<T, E>;
}

pub trait ReadSensor {
    fn read_sensor(&self) -> SensorData;
}

pub trait GetFormat {
    fn get_format(&self) -> &'static str;
}

pub trait GetNames {
    fn get_names(&self) -> &'static str;
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


