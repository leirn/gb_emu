#[derive(Default)]
pub struct Controller {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,

    direction_keys_selected: bool,
    button_keys_selected: bool,

    interrrupted: bool,
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
            direction_keys_selected: true,
            interrrupted: false,
        }
    }
}

impl Controller {
    pub fn get_controller_status(&self) -> u8 {
        let low_nibble_button = if self.button_keys_selected {
            ((self.start as u8) << 3)
                | ((self.select as u8) << 2)
                | ((self.b as u8) << 1)
                | (self.a as u8)
        } else {
            0
        };

        let low_nibble_direction = if self.button_keys_selected {
            ((self.down as u8) << 3)
                | ((self.up as u8) << 2)
                | ((self.left as u8) << 1)
                | (self.right as u8)
        } else {
            0
        };

        low_nibble_direction
            | low_nibble_button
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

    pub fn press_a(&mut self) {
        if self.button_keys_selected {
            self.a = true;
            self.interrrupted = true;
        }
    }

    pub fn press_b(&mut self) {
        if self.button_keys_selected {
            self.a = true;
            self.interrrupted = true;
        }
    }

    pub fn press_start(&mut self) {
        if self.button_keys_selected {
            self.a = true;
            self.interrrupted = true;
        }
    }

    pub fn press_select(&mut self) {
        if self.button_keys_selected {
            self.a = true;
            self.interrrupted = true;
        }
    }

    pub fn unpress_a(&mut self) {
        self.a = false;
    }

    pub fn unpress_b(&mut self) {
        self.b = false;
    }

    pub fn unpress_start(&mut self) {
        self.start = false;
    }

    pub fn unpress_select(&mut self) {
        self.select = false;
    }

    pub fn press_up(&mut self) {
        if self.direction_keys_selected {
            self.up = true;
            self.interrrupted = true;
        }
    }

    pub fn press_down(&mut self) {
        if self.direction_keys_selected {
            self.down = true;
            self.interrrupted = true;
        }
    }

    pub fn press_left(&mut self) {
        if self.direction_keys_selected {
            self.left = true;
            self.interrrupted = true;
        }
    }

    pub fn press_right(&mut self) {
        if self.direction_keys_selected {
            self.right = true;
            self.interrrupted = true;
        }
    }

    pub fn unpress_up(&mut self) {
        self.up = false;
    }
    pub fn unpress_down(&mut self) {
        self.down = false;
    }
    pub fn unpress_left(&mut self) {
        self.left = false;
    }
    pub fn unpress_right(&mut self) {
        self.right = false;
    }

    pub fn is_interrupted(&mut self) -> bool {
        let value = self.interrrupted;
        self.interrrupted = false;
        value
    }
}
