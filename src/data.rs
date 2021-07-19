use std::env;

fn get_asset() {
    let texture: Texture2D = match load_texture("assets/ferris.png").await {
        Ok(texture) => texture,
        Err(e) => {
            let path = env::current_dir().unwrap();
            eprintln!("The current directory is {}", path.display());
            eprintln!("path: {}", e.path);
            panic!("cant open texture")
        }
    };
}
