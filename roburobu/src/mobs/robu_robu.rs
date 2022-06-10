use raylib::prelude::*;

use super::Mob;

#[derive(PartialEq)]
enum Jump {
    OnGround,
    Jumping(bool),
}

#[allow(dead_code)] //temporary allow dead code
pub struct RobuRobu {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    jumping: Jump,
}

impl Mob for RobuRobu {

    fn update(&mut self) {
        
        self.jump()

    }

    fn key_press(&mut self, rl: &RaylibHandle) {
        
        if (rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_key_pressed(KeyboardKey::KEY_UP)) && self.jumping == Jump::OnGround {
            self.jumping = Jump::Jumping(true);
            println!("jump jump jump")
        }

    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        
        d.draw_rectangle(self.x, self.y, self.w, self.h, Color::BLACK)

    }
}

impl RobuRobu {
    pub fn create(x: i32, y: i32, _rl: &mut RaylibHandle) -> Self {
        Self {
            x,
            y,
            w: 50,
            h: 100,
            jumping: Jump::OnGround
        }
    }

    fn jump(&mut self) {

        match self.jumping {
            Jump::OnGround => return,
            Jump::Jumping(jumping) => {
                                                if self.y > 190 && jumping {
                                                    self.y -= 5;
                                                } 
                                            
                                                if self.y < 320 && !jumping{
                                                    self.y += 5;
                                                } 
                                                
                                                if self.y <= 190 {
                                                    self.jumping = Jump::Jumping(false);
                                                }
                                                if self.y >= 320 && !jumping {
                                                    self.jumping = Jump::OnGround;
                                                }
                                            }
                                            
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
