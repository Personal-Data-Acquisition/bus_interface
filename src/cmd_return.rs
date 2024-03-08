// The data that gets returned from the command requests.
#![allow(dead_code)]
#[derive(Debug)]
pub struct CmdReturn {
    name: String,
    format: String,
    data_names: String,
    raw_bytes: Vec<u8>,
}

impl CmdReturn {
    pub fn new() -> CmdReturn {
        let ret = CmdReturn{
            name: String::new(),
            format: String::new(),
            data_names: String::new(),
            raw_bytes: vec![],
        };
        ret
    }
    

}

//Tests for the structure
mod test_cmdreturn {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn self_test() {
        assert!(true);
    }
}
