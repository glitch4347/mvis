use std::env;

use macroquad::prelude::*;


struct Vis {
    pos: Vec2
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

    let vis = Vis {
        pos: Vec2::new(screen_width() / 2.,  screen_height() / 2.) 
    };



    loop {
        clear_background(LIGHTGRAY);

        draw_circle(vis.pos.x, vis.pos.y, 2., BLACK);

        next_frame().await
    }
}