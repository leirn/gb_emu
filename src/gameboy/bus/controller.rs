#[derive(Default)]
pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,

    pub button_keys_selected: bool,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            up: true,
            down: true,
            left: true,
            right: true,
            a: true,
            b: true,
            start: true,
            select: true,

            button_keys_selected: true,
        }
    }
}

impl Controller {
    pub fn get_controller_status(&self) -> u8 {
        let low_nibble = if self.button_keys_selected {
            ((self.start as u8) << 3)
                | ((self.select as u8) << 2)
                | ((self.b as u8) << 1)
                | (self.a as u8)
        } else {
            ((self.down as u8) << 3)
                | ((self.up as u8) << 2)
                | ((self.left as u8) << 1)
                | (self.right as u8)
        };

        low_nibble
            | ((self.button_keys_selected as u8) << 5)
            | (((!self.button_keys_selected) as u8) << 4)
    }

    pub fn set_controller_status(&mut self, value: u8) {
        if (value >> 5) & 1 == 1 {
            self.button_keys_selected = true;
        } else {
            self.button_keys_selected = false;
        }
    }
}
