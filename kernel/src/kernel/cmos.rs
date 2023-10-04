use super::asm::{self, in_u8};

pub struct Cmos {
    pub raw: [u8; 128],
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
    pub century: u8,
    pub register_a: u8,
    pub register_b: u8,
}

impl Cmos {
    pub fn new() -> Self {
        Self {
            raw: [0; 128],
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
            century: 0,
            register_b: 0,
            register_a: 0,
        }
    }

    pub fn read(bcd_to_bin: bool) -> Self {
        // wait if CMOS is updating
        // while is_updating() {}
        let mut array: [u8; 128] = [0; 128];
        // let mut index = 0;
        // while index < 128 {
        //     asm::out_u16(0x70, index);
        //     array[index as usize] = asm::in_u8(0x71);
        //     index += 1;
        // }

        // // convert bcd to binary
        // if (array[7] & 0x04) == 0 && bcd_to_bin {
        //     array[0] = ((array[0] & 0x0F) + (((array[0] & 0x70) / 16) * 10)) | (array[0] & 0x80);
        //     array[1] = ((array[1] & 0x0F) + (((array[1] & 0x70) / 16) * 10)) | (array[1] & 0x80);
        //     array[2] = ((array[2] & 0x0F) + (((array[2] & 0x70) / 16) * 10)) | (array[2] & 0x80);
        //     array[3] = ((array[3] & 0x0F) + (((array[3] & 0x70) / 16) * 10)) | (array[3] & 0x80);
        //     array[4] = ((array[4] & 0x0F) + (((array[4] & 0x70) / 16) * 10)) | (array[4] & 0x80);
        //     array[5] = ((array[5] & 0x0F) + (((array[5] & 0x70) / 16) * 10)) | (array[5] & 0x80);
        // }

        // // convert 12 hour clock to 24 if needed
        // if (array[7] & 0x02) == 0 && (array[2] & 0x80) == 1 {
        //     array[2] = ((array[2] & 0x7F) + 12) % 24;
        // }
        Self {
            raw: array,
            second: array[0],
            minute: array[1],
            hour: array[2],
            day: array[3],
            month: array[4],
            year: array[5],
            century: array[6],
            register_b: array[7],
            register_a: array[8],
        }
    }
}

unsafe fn is_updating() -> bool {
    asm::out_u16(0x70, 0x0A);
    (in_u8(0x71) & 0x80) == 1
}
