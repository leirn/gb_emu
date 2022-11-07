// http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt
// http://bgb.bircd.org/pandocs.htm#videodisplay

use std::fmt;

const NINTENDO_LOGO_SIZE: usize = 0x30;

const NINTENDO_LOGO: [u8; NINTENDO_LOGO_SIZE] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

const VRAM_SIZE: usize = 0x2000;
pub const OAM_SIZE: usize = 0x9f;

enum SpriteSize {
    Size8x8,
    Size8x16,
}

#[derive(PartialEq, Debug)]
enum State {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    x: u8,
    y: u8,
    state: State,
    /// Bit 7 - LCD Display Enable             (0=Off, 1=On)
    /// Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    /// Bit 5 - Window Display Enable          (0=Off, 1=On)
    /// Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
    /// Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
    /// Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
    /// Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
    /// Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
    lcd_control: u8,
    lcd_status: u8,
    scroll_x: u8,
    scroll_y: u8,
    lyc: u8,
    bg_palette_data: u8,
    object_palette_0_data: u8,
    object_palette_1_data: u8,
    window_y_position: u8,
    window_x_position_minus_7: u8,
}

impl fmt::Display for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X:{}, Y:{}, state:{}", self.x, self.y, self.state)
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            vram: [(); VRAM_SIZE].map(|_| 0),
            oam: [0; OAM_SIZE],
            x: 0,
            y: 0,
            state: State::OAMSearch,
            lcd_control: 0,
            lcd_status: 0,
            scroll_x: 0,
            scroll_y: 0,
            lyc: 0,
            bg_palette_data: 0,
            object_palette_0_data: 0,
            object_palette_1_data: 0,
            window_y_position: 0,
            window_x_position_minus_7: 0,
        }
    }

    pub fn next(&mut self) {
        // TODO : compare LY and LYC?

        self.x += 1;
        if self.x == 160 {
            self.x = 0;
            self.y += 1;
            if self.y == 144 {
                self.state = State::VBlank;
            } else if self.y == 153 {
                self.y = 0;
                self.state = State::OAMSearch
            }
        } else if self.x == 20 && self.state == State::OAMSearch {
            self.state = State::PixelTransfer;
        } else if self.x == 20 && self.state == State::PixelTransfer {
            self.state = State::HBlank;
        }
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;
    }

    pub fn read_oam(&self, address: usize) -> u8 {
        self.oam[address]
    }

    pub fn write_oam(&mut self, address: usize, value: u8) {
        self.oam[address] = value;
    }

    pub fn read_registers(&self, address: u16) -> u8 {
        match address {
            0xff40 => self.lcd_control,
            0xff41 => self.lcd_status,
            0xff42 => self.scroll_x,
            0xff43 => self.scroll_y,
            0xff44 => self.y,
            0xff45 => self.lyc,
            0xff47 => self.bg_palette_data,
            0xff48 => self.object_palette_0_data,
            0xff49 => self.object_palette_1_data,
            0xff4a => self.window_y_position,
            0xff4b => self.window_x_position_minus_7,
            _ => panic!("Not a PPU register address or write-only"),
        }
    }

    pub fn write_registers(&mut self, address: u16, value: u8) {
        match address {
            0xff40 => self.lcd_control = value,
            0xff41 => {
                self.lcd_status =
                    (value & 0xfb) | (self.get_lyc_ly_coincidence() as u8) << 2 | self.get_state()
            }
            0xff42 => self.scroll_x = value,
            0xff43 => self.scroll_y = value,
            0xff45 => self.lyc = value,
            0xff47 => self.bg_palette_data = value,
            0xff48 => self.object_palette_0_data = value,
            0xff49 => self.object_palette_1_data = value,
            0xff4a => self.window_y_position = value,
            0xff4b => self.window_x_position_minus_7 = value,
            _ => panic!("Not a PPU register address or read-only"),
        }
    }

    fn get_lyc_ly_coincidence(&self) -> bool {
        if (self.lcd_status >> 6) & 0x1 == 0x1 {
            self.lyc == self.y
        } else {
            self.lyc != self.y
        }
    }

    fn get_state(&self) -> u8 {
        match self.state {
            State::HBlank => 0,
            State::VBlank => 1,
            State::OAMSearch => 2,
            State::PixelTransfer => 3,
        }
    }

    pub fn set_oam(&mut self, data: [u8; OAM_SIZE]) {
        self.oam = data.clone();
    }

    fn is_lcd_display_enabled(&self) -> bool {
        (self.lcd_control >> 7) == 0x01
    }

    fn get_window_tile_map_display_selected(&self) -> u16 {
        if (self.lcd_control >> 6) & 0x1 == 0x1 {
            0x9c00
        } else {
            0x9800
        }
    }

    fn is_window_display_enabled(&self) -> bool {
        (self.lcd_control >> 5) & 0x1 == 0x1
    }

    fn get_bg_and_window_tile_dara_selected(&self) -> u16 {
        if (self.lcd_control >> 4) & 0x1 == 0x1 {
            0x8000
        } else {
            0x8800
        }
    }

    fn get_bg_tile_map_display_selected(&self) -> u16 {
        if (self.lcd_control >> 3) & 0x1 == 0x1 {
            0x9c00
        } else {
            0x9800
        }
    }

    fn get_sprite_obj_size(&self) -> SpriteSize {
        if (self.lcd_control >> 2) & 0x1 == 0x1 {
            SpriteSize::Size8x16
        } else {
            SpriteSize::Size8x8
        }
    }

    fn is_sprite_obj_enabled(&self) -> bool {
        (self.lcd_control >> 1) & 0x1 == 0x1
    }

    fn is_bg_enabled(&self) -> bool {
        self.lcd_control & 0x1 == 0x1
    }
}
