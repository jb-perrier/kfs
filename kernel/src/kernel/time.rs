use super::cmos::Cmos;

pub struct Time {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Time {
    pub const fn new() -> Self {
        Self {
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        }
    }

    pub unsafe fn from_cmos(cmos: Cmos) -> Self {
        // rebuild 4-digits year
        let mut year = (2023 / 100) * 100;
        if year < 2023 {
            year += 100;
        }
        Self {
            second: cmos.second,
            minute: cmos.minute,
            hour: cmos.hour,
            day: cmos.day,
            month: cmos.month,
            year,
        }
    }
}
