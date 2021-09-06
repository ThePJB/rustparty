use crate::rect::Rect;
use crate::vec3::Vec3;

pub struct Canvas<'a> {
    sdl_canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
}

fn v3_to_sdl_colour(v: Vec3) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB((v.x * 255.0) as u8, (v.y * 255.0) as u8, (v.z * 255.0) as u8)
}

fn rect_to_sdl_rect(screen_x: u32, screen_y: u32, r: Rect) -> sdl2::rect::Rect {
    // map x from 0..a
    // map y from 0..1
    let a = screen_x as f32 / screen_y as f32;
    let ret = sdl2::rect::Rect::new(
        (screen_x as f32 * r.x / a) as i32,
        (screen_y as f32 * r.y) as i32,
        (screen_x as f32 * r.w / a) as u32, 
        (screen_y as f32 * r.h) as u32,
    );
    return ret;
}

impl Canvas<'_> {

    pub fn new(sdl_canvas: &'_ mut sdl2::render::Canvas<sdl2::video::Window>) -> Canvas<'_> {
        Canvas{sdl_canvas: sdl_canvas}
    }

    pub fn clear(&mut self, colour: Vec3) {
        self.sdl_canvas.set_draw_color(v3_to_sdl_colour(colour));
        self.sdl_canvas.clear();

    }

    pub fn draw_rect(&mut self, r: Rect, colour: Vec3) {
        self.sdl_canvas.set_draw_color(v3_to_sdl_colour(colour));
        let (screen_x, screen_y) = self.sdl_canvas.output_size().unwrap();
        let sdl_rect = rect_to_sdl_rect(screen_x, screen_y, r);
        self.sdl_canvas.fill_rect(sdl_rect);
    }

    pub fn present(&mut self) {
        self.sdl_canvas.present();
    }
}