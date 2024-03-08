// The data that gets returned from the command requests.
#![allow(dead_code)]
#[derive(Debug)]
pub struct CmdReturn {
    name: String,
    format: String,
    data_names: Vec<String>,
    raw_bytes: Vec<u8>,
}

impl CmdReturn {
    pub fn new() -> CmdReturn {
        let ret = CmdReturn{
            name: String::new(),
            format: String::new(),
            data_names: vec![], 
            raw_bytes: vec![],
        };
        ret
    }  

    pub fn parse_to_json(&self) -> String {
        let mut json = String::new();
        //1. add the name json.
        json.push_str("{\"name\":\"");
        json.push_str(&self.name);
        json.push_str("\", ");

        //2. Add the data.
        //. figure out how many vars of data we have.
        json
    }

    fn num_vars(&self) -> usize {
        return 0    
    }

}

//Tests for the structure
mod test_cmdreturn {
    #![allow(unused_imports)]
    use super::*;
    
    fn setup() -> CmdReturn {
        let mut new_response = CmdReturn::new();
        new_response.name = String::from("aht20");
        new_response.format = String::from("u16 u16");
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
    fn test_parse_to_json() {
        let ret = setup();
        let correct_response = String::from("{\"name\":\"aht20\", \"Temp\":\"255\", \"Humid\":\"255\"}");
        
        //test list
        //1. has parse function.
        let json_str = ret.parse_to_json();

        //2. outputs jason correctly.
        assert_eq!(json_str, correct_response);
    }
}
