// The data that gets returned from the command requests.
#![allow(dead_code)]


//considering the use of this C style union.
//it requires use of 'unsafe' and may not be the best choice.
#[repr(C)]
union UData {
    du8: u8,
    du16: u16,
    du32: u32,
    di8: i8,
    di16: i16,
    di32: i32,
    df32: f32,
}

#[derive(Debug)]
pub struct CmdReturn {
    pub name: String,
    pub format: Vec<String>,
    pub data_names: Vec<String>,
    pub raw_bytes: Vec<u8>,
}

impl CmdReturn {
    pub fn new() -> CmdReturn {
        let ret = CmdReturn{
            name: String::new(),
            format: vec![],
            data_names: vec![], 
            raw_bytes: vec![],
        };
        ret
    }

    pub fn parse_raw_to_format(&mut self) {

    }


    fn bytes_to_strings(&self) -> Vec<String> {
        let mut data: Vec<String> = vec![]; 
        let mut byte_index: usize = 0;
        
        for fmt in &self.format {
            if fmt.contains("u8") {
                data.push(format!("{}",self.raw_bytes[byte_index]));    
                byte_index += 1;
            }
            else if fmt.contains("u16") {
                let tmp: u16 = bytes_to_u16(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp));    
                byte_index += 2;
            }

            else if fmt.contains("i16") {
                let tmp: i16 = bytes_to_i16(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp));    
                byte_index += 2;
            }

            else if fmt.contains("u32") {
                let tmp: u32 = bytes_to_u32(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp)); 
                byte_index += 4;
            }

            else if fmt.contains("i32") {
                let tmp: i32 = bytes_to_i32(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp)); 
                byte_index += 4;
            }
        }

        data
    }

}

fn bytes_to_u16(b: &Vec<u8>, start: usize) -> u16 {
        let tmp: u16 =
            ((b[start] as u16 )<< 8) | 
            (b[start + 1] as u16);
        tmp
}


fn bytes_to_i16(b: &Vec<u8>, start: usize) -> i16 {
        let tmp: i16 =
            ((b[start] as i16 )<< 8) | 
            (b[start + 1] as i16);
        tmp
}


fn bytes_to_u32(b: &Vec<u8>, start: usize) -> u32 {
    let tmp: u32 = 
        ((b[start] as u32 )<< 24) | 
        ((b[start + 1] as u32 )<< 16) | 
        ((b[start + 2] as u32 )<< 8) | 
        (b[start + 3] as u32);
        tmp
}


fn bytes_to_i32(b: &Vec<u8>, start: usize) -> i32 {
    let tmp: i32 = 
        ((b[start] as i32 )<< 24) | 
        ((b[start + 1] as i32 )<< 16) | 
        ((b[start + 2] as i32 )<< 8) | 
        (b[start + 3] as i32);
        tmp
}

//Tests for the structure
mod test_cmdreturn {
    #![allow(unused_imports)]
    use super::*;
    
    fn setup() -> CmdReturn {
        let mut new_response = CmdReturn::new();
        new_response.name = String::from("aht20");
        new_response.format.push(String::from("u8"));
        new_response.format.push(String::from("u16"));
        new_response.format.push(String::from("u16"));
        new_response.data_names.push(String::from("Status"));
        new_response.data_names.push(String::from("Temp"));
        new_response.data_names.push(String::from("Humid"));
        new_response.raw_bytes = vec!(0, 255, 0, 255);

        new_response
    }

    #[test]
    fn self_test() {
        assert!(true);
    }


    #[test]
    fn test_name() {
        let mut new_response = CmdReturn::new();
        new_response.name = String::from("fake_sensor");
        assert_eq!(new_response.name, String::from("fake_sensor"));
    }


    #[test]
    fn test_parse_raw_to_format() {
        
        //setup the conditions for the test
        let mut ret = setup();
        ret.format = vec![];
        ret.raw_bytes = String::from("u8 u16 u16").into_bytes();
       
        //check that it's currently empty.
        assert_eq!(ret.format.len(), 0);

        //call the cut(code under test)
        ret.parse_raw_to_format();

        //check that it parses correctly
        assert_eq!(ret.format.len(), 3);
        assert_eq!(ret.format[0], String::from("u8"));
        assert_eq!(ret.format[1], String::from("u16"));
        assert_eq!(ret.format[2], String::from("u16"));

        //clean up 
    }
}
