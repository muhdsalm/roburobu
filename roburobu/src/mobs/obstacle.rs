use raylib::prelude::*;

use super::Mob;

#[allow(dead_code)] //temporary allow dead code
pub struct Obstacle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Mob for Obstacle {

    fn update(&mut self) {
        
        self.x -= 5;

    }
    fn key_press(&mut self, rl: &RaylibHandle) {


    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        
        d.draw_rectangle(self.x, self.y, self.w, self.h, Color::BLACK)

    }
}

impl Obstacle{
    pub fn create(x: i32, y: i32, _rl: &mut RaylibHandle) -> Self {
        Self {
            x,
            y,
            w: 25,
            h: 50,
        }
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.x as f32,
            y: self.y as f32,
            width: self.w as f32,
            height: self.h as f32,

        }
    }

}
