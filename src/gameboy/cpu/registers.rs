#[derive(PartialEq)]
pub enum RegisterNames {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    IndirectHL,
}

#[derive(Default, Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn inc_bc(&mut self) {
        self.set_bc(self.get_bc() + 1);
    }

    pub fn dec_bc(&mut self) {
        self.set_bc(self.get_bc() - 1);
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn inc_de(&mut self) {
        self.set_de(self.get_de() + 1);
    }

    pub fn dec_de(&mut self) {
        self.set_de(self.get_de() - 1);
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }

    pub fn inc_hl(&mut self) {
        self.set_hl(self.get_hl() + 1);
    }

    pub fn dec_hl(&mut self) {
        self.set_hl(self.get_hl() - 1);
    }

    pub fn inc_sp(&mut self) {
        self.sp += 1;
    }

    pub fn dec_sp(&mut self) {
        self.sp -= 1;
    }
}
