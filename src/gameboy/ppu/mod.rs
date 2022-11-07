// http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt

use std::fmt;

const NINTENDO_LOGO_SIZE: usize = 0x30;

const NINTENDO_LOGO: [u8; NINTENDO_LOGO_SIZE] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x9f;

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
        }
    }

    pub fn next(&mut self) {
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
}
