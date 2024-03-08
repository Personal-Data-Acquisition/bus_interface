pub trait BlockingCan {
    type Frame: Frame;
    type Error: Error;
    
    // Required methods
    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;
    fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}



struct CanFrameStd {
    sof: bool,
    std_id: u16,
    rtr: bool,
    ide: bool
    rb0: bool
    dlc: u8,
    data: [u8; 8],
    crc: u16,
}
