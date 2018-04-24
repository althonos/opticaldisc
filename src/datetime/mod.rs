#[derive(Clone, Debug, PartialEq)]
pub struct Datetime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub hundredth: u8,
    pub tz: u8,
}
