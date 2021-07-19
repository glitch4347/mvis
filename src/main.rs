use core::f32;
use std::env;

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
        let radius = screen_width().min(screen_height()) / 2.;

        

        for i in 0..bars_count {
            let rotation = i as f32 * 20.;
            bars.push(Bar::new(pos, rotation, radius))
        }
        Vis{
            pos,
            bars,
            radius
        }

    }

    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., BLACK);
        for b in &self.bars {
            b.render();
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
    fn render(&self) {
        let posx = self.radius * self.rotation.cos() + self.origin.x;
        let posy = self.radius * self.rotation.sin() + self.origin.y;
        draw_circle(posx, posy, 10., BLACK);

    }
}


#[macroquad::main("Texture")]
async fn main() {

    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    let vis = Vis::new(5);


    loop {
        clear_background(LIGHTGRAY);

        vis.render();

        next_frame().await
    }
}