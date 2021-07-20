use core::f32;
use std::{env, ops::Mul};

use macroquad::prelude::*;


// TODO: moving wave 
// TODO: curcles around

struct Vis {
    pos: Vec2,
    bars: Vec<Bar>,
    radius: f32
}

impl Vis {
    fn new(bars_count: u32) -> Vis {
        let mut bars = Vec::<Bar>::new();

        let pos = Vec2::new(screen_width() / 2.,  screen_height() / 2.);
        let radius = screen_width().min(screen_height()) / 3.;

        for i in 0..bars_count {
            let length = 180. / bars_count as f32;
            let rotation = i as f32 * length;
            bars.push(Bar::new(pos, rotation.to_radians(), radius))
        }
        Vis{
            pos,
            bars,
            radius
        }

    }

    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., BLACK);
        let mut pulse = 1.;
        for b in &self.bars {
            b.render(pulse);
            pulse += 1.5;
        }
    }
}

struct Bar {
    origin: Vec2,
    rotation: f32,
    radius: f32
}

impl Bar {
    fn new(origin: Vec2, rotation: f32, radius: f32) -> Bar {
        Bar { origin, rotation, radius }
    }
    fn render(&self, pulse: f32) {
        let x0 = self.radius * self.rotation.cos() + self.origin.x;
        let y0 = self.radius * self.rotation.sin() + self.origin.y;
        draw_circle(x0, y0, 5., BLACK);

        let x1 = x0 + self.rotation.cos() * pulse / 2.;
        let y1 = y0 + self.rotation.sin() * pulse / 2.;
        let x2 = x0 + self.rotation.cos() * -pulse / 2.;
        let y2 = y0 + self.rotation.sin() * -pulse / 2.;

        draw_line(x1, y1, x2, y2, 2., GREEN);
    }
}


#[macroquad::main("Texture")]
async fn main() {

    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    let vis = Vis::new(35);


    loop {
        clear_background(LIGHTGRAY);

        vis.render();

        next_frame().await
    }
}