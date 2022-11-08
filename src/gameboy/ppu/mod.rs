// http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt
// http://bgb.bircd.org/pandocs.htm#videodisplay
// https://blog.tigris.fr/2019/09/15/writing-an-emulator-the-first-pixel/

mod screen;

use screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{fmt, u16};

const VRAM_SIZE: usize = 0x2000;
pub const OAM_SIZE: usize = 0x9f;
const TILE_SIZE: u16 = 16;
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

#[derive(Default)]
enum FetcherState {
    #[default]
    ReadTileID,
    ReadTileData0,
    ReadTileData1,
    PushToFIFO,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default)]
struct PixelFetcher {
    tick: u16,
    state: FetcherState,
    tile_index: u8,
    map_addr: u16,
    tile_line: u8,
    tile_id: u8,
    pixel_data: [u8; 8],
    fifo: VecDeque<u8>,
}

pub struct Ppu<'a> {
    screen: Screen<'a>,
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    tick: u16,
    x: u8,
    y: u8,
    state: State,
    fetcher: PixelFetcher,
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

impl fmt::Display for Ppu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X:{}, Y:{}, state:{}", self.x, self.y, self.state)
    }
}

impl Ppu<'_> {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Ppu<'static> {
        let mut ppu = Ppu {
            screen: Screen::new(sdl_context),
            vram: [(); VRAM_SIZE].map(|_| 0),
            oam: [0; OAM_SIZE],
            tick: 0,
            fetcher: PixelFetcher::default(),
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
        };
        ppu.screen.start();
        ppu
    }

    // https://blog.tigris.fr/2019/09/15/writing-an-emulator-the-first-pixel/
    pub fn next(&mut self) {
        self.tick += 1;

        match self.state {
            State::OAMSearch => {
                if self.tick == 40 {
                    self.state = State::PixelTransfer;
                    self.x = 0;
                    let tile_line = self.y % 8;
                    let tile_map_base_addr = self.get_bg_tile_map_display_selected();
                    let tile_map_row_addr = tile_map_base_addr + ((self.y as u16 / 8) * 32);
                    self.fetcher_start(tile_map_row_addr, tile_line);
                }
            }
            State::PixelTransfer => {
                self.fetcher_next();

                if self.fetcher.fifo.len() > 0 {
                    let pixel_color = self.fetcher.fifo.pop_front().unwrap();
                    self.screen.set_pixel(self.x, self.y, pixel_color);
                    self.x += 1;
                }
                if self.x == 160 {
                    self.state = State::HBlank;
                }
            }
            State::HBlank => {
                if self.tick == 456 {
                    self.y += 1;
                    self.tick = 0;
                    if self.y == 144 {
                        self.state = State::VBlank;
                    } else {
                        self.state = State::OAMSearch;
                    }
                }
            }
            State::VBlank => {
                if self.tick == 456 {
                    self.y += 1;
                    self.tick = 0;
                }
                if self.y == 153 {
                    self.state = State::OAMSearch;
                    self.y = 0;
                    println!("Updating screen");
                    self.display_background_map();
                    self.screen.present(self.scroll_x, self.scroll_y);
                }
            }
        }
    }

    fn fetcher_start(&mut self, map_addr: u16, tile_line: u8) {
        self.fetcher.tile_index = 0;
        self.fetcher.map_addr = map_addr;
        self.fetcher.tile_line = tile_line;
        self.fetcher.tick = 0;
        self.fetcher.state = FetcherState::ReadTileID;
        self.fetcher.fifo.clear();
    }

    fn fetcher_next(&mut self) {
        // The Fetcher runs at half the speed of the PPU (every 2 clock cycles).
        self.fetcher.tick += 1;
        if self.fetcher.tick < 2 {
            return;
        }
        self.fetcher.tick = 0; // Reset tick counter and execute next state.

        match self.fetcher.state {
            FetcherState::ReadTileID => {
                self.fetcher.tile_id =
                    self.vram[self.fetcher.map_addr as usize + self.fetcher.tile_index as usize];
                self.fetcher.state = FetcherState::ReadTileData0;
            }

            FetcherState::ReadTileData0 => {
                let offset = self.get_bg_and_window_tile_data_selected()
                    + self.fetcher.tile_id as u16 * TILE_SIZE;

                let addr = offset + self.fetcher.tile_line as u16 * 2;

                let data = self.vram[addr as usize];

                for bitpos in 0..8 {
                    self.fetcher.pixel_data[bitpos] = (data >> bitpos) & 0x01;
                }

                self.fetcher.state = FetcherState::ReadTileData1;
            }

            FetcherState::ReadTileData1 => {
                let offset = self.get_bg_and_window_tile_data_selected()
                    + self.fetcher.tile_id as u16 * TILE_SIZE;

                let addr = offset + self.fetcher.tile_line as u16 * 2;

                let data = self.vram[addr as usize + 1];

                for bitpos in 0..8 {
                    self.fetcher.pixel_data[bitpos] = (data >> bitpos) & 0x01;
                }
                self.fetcher.state = FetcherState::PushToFIFO;
            }

            FetcherState::PushToFIFO => {
                for i in 0..8 {
                    self.fetcher.fifo.push_back(self.fetcher.pixel_data[7 - i]);
                }
                self.fetcher.tile_index += 1;
                self.fetcher.state = FetcherState::ReadTileID;
            }
        }
    }

    fn display_background_map(&mut self) {
        let base_address = self.get_bg_tile_map_display_selected() as usize;

        for y in 0..32 {
            for x in 0..32 {
                let tile_number = self.vram[base_address + 32 * y + x] as usize;
                let tile_data_base_address = self.get_bg_and_window_tile_data_selected() as usize;
                let tile_data_address = tile_data_base_address + 16 * tile_number;
                for x_tile in 0..2 {
                    for y_tile in 0..8 {
                        // let go trough tile
                        let byte = self.vram[tile_data_address + y_tile * 2 + x_tile];
                        for j in 0..4 {
                            // let go trough bit pairs in byte
                            let color = (byte >> ((j * 4) * 2)) & 0x03;

                            let x_pos = x * 8 + x_tile * 2 + j;
                            let y_pos = y * 8 + y_tile;

                            self.screen
                                .set_pixel(x_pos as u8, y_pos as u8, (color % 4) as u8);
                        }
                    }
                }
            }
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
            0x9c00 - 0x8000 // VRAM Offset substracted
        } else {
            0x9800 - 0x8000 // VRAM Offset substracted
        }
    }

    fn is_window_display_enabled(&self) -> bool {
        (self.lcd_control >> 5) & 0x1 == 0x1
    }

    fn get_bg_and_window_tile_data_selected(&self) -> u16 {
        if (self.lcd_control >> 4) & 0x1 == 0x1 {
            0x8000 - 0x8000 // VRAM Offset substracted
        } else {
            0x8800 - 0x8000 // VRAM Offset substracted
        }
    }

    fn get_bg_tile_map_display_selected(&self) -> u16 {
        if (self.lcd_control >> 3) & 0x1 == 0x1 {
            0x9c00 - 0x8000 // VRAM Offset substracted
        } else {
            0x9800 - 0x8000 // VRAM Offset substracted
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
