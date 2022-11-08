#[derive(Default, Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn get_flags(&self) -> u8 {
        ((self.zero as u8) << 7)
            | ((self.negative as u8) << 6)
            | ((self.half_carry as u8) << 5)
            | ((self.carry as u8) << 4)
    }

    pub fn set_flags(&mut self, flag: u8) {
        self.zero = ((flag >> 7) & 1) == 1;
        self.negative = ((flag >> 6) & 1) == 1;
        self.half_carry = ((flag >> 5) & 1) == 1;
        self.carry = ((flag >> 4) & 1) == 1;
    }

    pub fn clear_flags(&mut self) {
        self.zero = false;
        self.negative = false;
        self.half_carry = false;
        self.carry = false;
    }

    pub fn _set_nz(&mut self, value: u8) {
        self._set_negative(value);
        self.set_zero(value);
    }

    pub fn _set_negative(&mut self, value: u8) {
        self.negative = (value >> 7) != 0;
    }

    pub fn set_zero(&mut self, value: u8) {
        self.zero = value == 0;
    }

    pub fn set_half_carry_8(&mut self, a: u8, b: u8) {
        self.half_carry = (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10;
    }
    pub fn set_half_carry_16(&mut self, a: u16, b: u16) {
        self.half_carry = (((a & 0xfff) + (b & 0xfff)) & 0x1000) == 0x1000;
    }
}
