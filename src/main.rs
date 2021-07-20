use core::f32;
use std::{env, ops::Mul};

use macroquad::prelude::*;


mod bar;
use crate::bar::*;


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
            let length = 360. / bars_count as f32;
            let rotation = i as f32 * length;
            bars.push(Bar::new(pos, rotation.to_radians(), radius))
        }
        Vis{ pos, bars, radius }

    }

    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., GREEN);
        let mut pulse = 1.;
        for b in &self.bars {
            b.render(pulse);
            pulse += 1.5;
        }
    }
}



#[macroquad::main("Texture")]
async fn main() {

    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    let vis = Vis::new(35);


    loop {
        clear_background(BLACK);

        vis.render();

        next_frame().await
    }
}