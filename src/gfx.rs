use sdl2::render::Renderer;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Pixel {
    x: i32,
    y: i32,
    value: u8,
}

impl Pixel {
    pub fn new(x: i32, y: i32, value: u8) -> Pixel {
        Pixel {
            x: x,
            y: y,
            value: value,
        }
    }

    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn color(&self) -> Color {
        if self.value == 1 {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        }
    }
}

#[derive(Debug)]
pub struct Gfx {
    pub port: Receiver<Vec<Pixel>>,
}

impl Gfx {
    pub fn flush(&self, renderer: &mut Renderer) {
        match self.port.try_recv() {
            Ok(pixels) => {
                for pixel in pixels.iter() {
                    debug!("Rendering pixel {:?}", pixel);

                    let _ = renderer.set_draw_color(pixel.color());
                    let _ = renderer.draw_point(pixel.point());
                }

                let _ = renderer.present();
            },
            _ => {}, // TODO: Handle disconnections
        }
    }
}
