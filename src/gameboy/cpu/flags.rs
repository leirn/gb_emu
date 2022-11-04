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
}
