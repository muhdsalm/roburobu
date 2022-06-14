use raylib::prelude::*;



#[allow(dead_code)] //temporary allow dead code
#[derive(PartialEq, Clone, Copy)]
pub struct Obstacle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Obstacle {

    pub fn update(&mut self, score: i128) {
        
        self.x -= (5 + score / 100) as i32;



    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        
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
    pub fn get_x(&self) -> i32{
        self.x
    }

}
