use macroquad::prelude::*;

pub struct Bar {
    origin: Vec2,
    rotation: f32,
    radius: f32
}

impl Bar {
    pub fn new(origin: Vec2, rotation: f32, radius: f32) -> Bar {
        Bar { origin, rotation, radius }
    }
    pub fn render(&self, pulse: f32) {
        let x0 = self.radius * self.rotation.cos() + self.origin.x;
        let y0 = self.radius * self.rotation.sin() + self.origin.y;
        draw_circle(x0, y0, 5., GREEN);

        let x1 = x0 + self.rotation.cos() * pulse / 2.;
        let y1 = y0 + self.rotation.sin() * pulse / 2.;
        let x2 = x0 + self.rotation.cos() * -pulse / 2.;
        let y2 = y0 + self.rotation.sin() * -pulse / 2.;

        draw_line(x1, y1, x2, y2, 2., GREEN);
    }
}