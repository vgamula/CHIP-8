use sdl2::pixels::Color;
use sdl2::rect::Rect;

// TODO: Remove duplications
const GFX_WIDTH: usize = 64;
const GFX_HEIGHT: usize = 32;



pub struct VideoDriver {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    gfx: [[u8; GFX_WIDTH]; GFX_HEIGHT],
    draw_flag: bool,
}

impl VideoDriver {
    pub fn new(sdl: &sdl2::Sdl) -> VideoDriver {
        let video_subsystem = sdl.video().unwrap();
        let _window = video_subsystem
            .window("CHIP-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();


        let canvas = _window.into_canvas().build().unwrap();
        VideoDriver {
            canvas,
            gfx: [[0; GFX_WIDTH]; GFX_HEIGHT],
            draw_flag: false,
        }
    }

    pub fn update_screen(&mut self) {
        if !self.draw_flag {
            return;
        }

        let scale: i32 = 10;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let main_color = Color::RGB(255, 255, 255);
        self.canvas.set_draw_color(main_color);
        for y in 0..GFX_HEIGHT {
            for x in 0..GFX_WIDTH {
                if self.gfx[y][x] != 1 {
                    continue;
                }
                let rect = Rect::new(
                    x as i32 * scale,
                    y as i32 * scale,
                    scale as u32,
                    scale as u32,
                );
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present();

        self.draw_flag = false;
    }

    pub fn reinitialize_screen(&mut self) {
        self.gfx = [[0; GFX_WIDTH]; GFX_HEIGHT];
        self.draw_flag = true;
    }

    pub fn pixel_state(&self, y: usize, x: usize) -> u8 {
        self.gfx[y][x]
    }

    pub fn toggle_pixel_state(&mut self, y: usize, x: usize, color: u8) {
        self.gfx[y][x] ^= color;
    }

    pub fn set_draw_flag(&mut self, flag: bool) {
        self.draw_flag = flag;
    }
}


