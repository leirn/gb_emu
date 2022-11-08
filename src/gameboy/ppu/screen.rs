use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub const SCREEN_WIDTH: u32 = 256;
pub const SCREEN_HEIGHT: u32 = 256;

pub const TEXTURE_ASPECT_RATIO: f64 = SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64;

pub struct Screen<'a> {
    canvas: Canvas<Window>,
    surface: Surface<'a>,
    g_scaling_mode: ScalingMode,
}

impl Screen<'_> {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Screen<'static> {
        let _video_subsystem = sdl_context.borrow_mut().video().unwrap();
        let _window = _video_subsystem
            .window("Window", SCREEN_WIDTH, SCREEN_HEIGHT)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let mut _canvas = _window
            .into_canvas()
            .index(Screen::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        let mut _surface =
            Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::Index8).unwrap();
        let palette = Palette::with_colors(&PALETTE).unwrap();
        _surface.set_palette(&palette).unwrap();

        _canvas
            .window_mut()
            .set_size(
                (3 * SCREEN_WIDTH).try_into().unwrap(),
                (3 * SCREEN_HEIGHT).try_into().unwrap(),
            )
            .unwrap();

        Screen {
            canvas: _canvas,
            surface: _surface,
            g_scaling_mode: ScalingMode::ScalingModeAspectFit,
        }
    }

    /// Find SDL GL Driver to initiate SDL window
    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    /// Start the Screen component
    pub fn start(&mut self) {
        self.canvas.clear();
        self.present(0, 0);
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, color: u8) {
        let address = x as usize + y as usize * SCREEN_WIDTH as usize;
        self.surface.without_lock_mut().unwrap()[address] = color;
    }

    pub fn print_window_square(&mut self, x: u8, y: u8) {
        let rectangle = sdl2::rect::Rect::new(x as i32 - 1, y as i32 - 1, 162 * 3, 144 * 3);
        self.canvas.draw_rect(rectangle).unwrap();
    }

    /// Refresh the windows with the buffered canvas
    pub fn present(&mut self, x: u8, y: u8) {
        self.canvas.clear();
        let creator = self.canvas.texture_creator();
        let texture = self.surface.as_texture(&creator).unwrap();

        self.canvas
            .copy(&texture, None, self.canvas.viewport())
            .unwrap();
        self.print_window_square(x, y);
        self.canvas.present();
    }

    pub fn update_window_viewport(&mut self) {
        let (window_height, window_width) = self.canvas.window().size();

        // If the scaling mode is fullscreen, use the window size
        if self.g_scaling_mode == ScalingMode::ScalingModeFullscreen {
            self.canvas.viewport().set_x(0);
            self.canvas.viewport().set_y(0);
            self.canvas.viewport().set_width(window_width);
            self.canvas.viewport().set_height(window_height);
            return;
        }

        let texture_aspect_ratio;
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectCorrect {
            texture_aspect_ratio = 4 as f64 / 3 as f64;
        } else {
            texture_aspect_ratio = TEXTURE_ASPECT_RATIO;
        }

        let mut max_viewport_width = window_width;
        let mut max_viewport_height = window_height;

        // For "integer factor" scaling, pick the highest integer factor that fits into the window
        if self.g_scaling_mode == ScalingMode::ScalingModeIntegerFactor {
            max_viewport_width =
                ((window_width as f64 / SCREEN_WIDTH as f64) * SCREEN_WIDTH as f64) as u32;
            max_viewport_height =
                ((window_height as f64 / SCREEN_HEIGHT as f64) * SCREEN_HEIGHT as f64) as u32;
        }

        // If the resulting viewport is too small, do proportional scaling according to the window size
        if max_viewport_width == 0 {
            max_viewport_width = window_width;
        }
        if max_viewport_height == 0 {
            max_viewport_height = window_height;
        }

        let screen_aspect_ratio = window_width as f64 / window_height as f64;
        let mut should_preserve_width = texture_aspect_ratio > screen_aspect_ratio;

        // The only difference between aspect fill and fit is that fit will leave black bars
        // and fill will crop the image.
        // TODO : does not seem to work, always in fit mode without preserving ratio
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectFill {
            should_preserve_width = !should_preserve_width;
        }

        if should_preserve_width {
            self.canvas
                .viewport()
                .set_x((window_width as i32 - max_viewport_width as i32) >> 1);
            self.canvas.viewport().set_width(max_viewport_width);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_height(viewport_width / texture_aspect_ratio as u32);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_y((window_height as i32 - viewport_height as i32) >> 1);
        } else {
            self.canvas
                .viewport()
                .set_y((window_height as i32 - max_viewport_height as i32) >> 1);
            self.canvas.viewport().set_height(max_viewport_height);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_width(viewport_height * texture_aspect_ratio as u32);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_x((window_width as i32 - viewport_width as i32) >> 1);
        }

        self.canvas.present();
    }

    pub fn get_scaling_mode(&self) -> ScalingMode {
        self.g_scaling_mode
    }

    pub fn set_scaling_mode(&mut self, mode: ScalingMode) {
        self.g_scaling_mode = mode;
        self.update_window_viewport();
    }
}

/// NES color palette
const PALETTE: [Color; 4] = [
    Color::RGB(0xff, 0xff, 0xff),
    Color::RGB(0x80, 0x80, 0x80),
    Color::RGB(0x40, 0x40, 0x40),
    Color::RGB(0x05, 0x05, 0x05),
];

#[derive(PartialEq, Clone, Copy)]
enum ScalingMode {
    ScalingModeAspectFit,
    ScalingModeAspectFill,
    ScalingModeIntegerFactor,
    ScalingModeFullscreen,
    ScalingModeAspectCorrect,
    ScalingModeCount,
}
