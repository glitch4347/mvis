use std::env;

use macroquad::prelude::*;


struct Vis {
    pos: Vec2,
    bars: Vec<Bar>
}

impl Vis {
    fn new() -> Vis {
        Vis{
            pos: Vec2::new(screen_width() / 2.,  screen_height() / 2.),
            bars: Vec::new()
        }
    }

    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., BLACK);
    }
}

struct Bar {
    origin: Vec2,
    rotation: f32
}


#[macroquad::main("Texture")]
async fn main() {

    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    let vis = Vis::new();


    loop {
        clear_background(LIGHTGRAY);

        vis.render();

        next_frame().await
    }
}