#[derive(Debug, PartialEq)] // to be able to assert equality for testing purposes
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}
