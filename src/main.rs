use std::env;

use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {

    for argument in env::args() {
        println!("{}", argument);
    }

    let texture: Texture2D = match load_texture("assets/ferris.png").await {
        Ok(texture) => texture,
        Err(e) => {
            let path = env::current_dir().unwrap();
            eprintln!("The current directory is {}", path.display());
            eprintln!("path: {}", e.path);
            panic!("cant open texture")
        }
    };

    loop {
        clear_background(LIGHTGRAY);
        draw_texture(
            texture,
            screen_width() / 2. - texture.width() / 2.,
            screen_height() / 2. - texture.height() / 2.,
            WHITE,
        );
        next_frame().await
    }
}