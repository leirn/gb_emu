#[derive(Default)]
pub struct Timer {
    divider_register: u16,
    time_counter: u8,
    time_modulo: u8,
    time_enable: bool,
    clock_mode: u8,
}

impl Timer {
    pub fn tick(&mut self) {
        self.divider_register = self.divider_register.wrapping_add(1);
    }

    pub fn get_div(&self) -> u8 {
        (self.divider_register >> 8) as u8
    }

    pub fn set_div(&mut self, value: u8) {
        self.divider_register = 0;
    }

    pub fn get_tima(&self) -> u8 {
        self.time_counter
    }

    pub fn set_tima(&mut self, value: u8) {
        self.time_counter = 0;
    }

    pub fn get_tma(&self) -> u8 {
        self.time_modulo
    }

    pub fn set_tma(&mut self, value: u8) {
        self.time_modulo = 0;
    }

    pub fn get_tac(&self) -> u8 {
        self.time_modulo
    }

    pub fn set_tac(&mut self, value: u8) {
        self.time_enable = value >> 2 & 0x1 == 0x1;
        self.clock_mode = value & 0x3;
    }
}

const CLOCK_SELECT: InputClockSelect = [1024, 16, 64, 256];

type InputClockSelect = [u16; 4];
