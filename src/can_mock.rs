pub trait BlockingCan {
    type Frame: Frame;
    type Error: Error;
    
    // Required methods
    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;
    fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}
