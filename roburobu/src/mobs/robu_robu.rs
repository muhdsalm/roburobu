use raylib::{prelude::*, misc::AsF32};



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
    robu_robu: Texture2D,
    ground_level: i32,
    texture_x: f32,
    texture_y: f32
}

impl RobuRobu {

    pub fn update(&mut self) {
        
        self.jump();
        self.texture_x += 1.;

        if self.texture_x == 3. {
            self.texture_x = 0.;
            self.texture_y += 1.;
        }
        if self.texture_y == 2. && self.texture_x == 2. {
                self.texture_y = 0.;
                self.texture_x = 0.;
        }

    }

    pub fn key_press(&mut self, rl: &RaylibHandle) {
        
        if (rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_key_pressed(KeyboardKey::KEY_UP)) && self.jumping == Jump::OnGround {
            self.jumping = Jump::Jumping(true);
            println!("jump jump jump")
        }

    }
    #[allow(dead_code, unused_variables)]

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        //d.draw_rectangle(self.x, self.y, self.w, self.h, Color::BLACK);
        d.draw_texture_rec(&self.robu_robu, Rectangle{x: ((self.robu_robu.width/3) as f32) * self.texture_x, y: ((self.robu_robu.height/3) as f32) * self.texture_y, width: (self.robu_robu.width/3).as_f32(), height: (self.robu_robu.height/3).as_f32()}, Vector2{x: self.x as f32, y: self.y as f32}, Color::WHITE);

    }
}

impl RobuRobu {
    pub fn create(x: i32, y: i32, _rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut guy = Self {
            x,
            y,
            w: 72,
            h: 100,
            jumping: Jump::OnGround,
            robu_robu: _rl.load_texture(thread, "src/resources/robu robu running.png").unwrap(),
            ground_level: 0,
            texture_x: 0.,
            texture_y: 0.,
        };
        guy.ground_level = guy.y;
        guy
    }

    fn jump(&mut self) {

        let y = self.ground_level;
        let jump_height = 150;

        match self.jumping {

            Jump::OnGround => return,
            Jump::Jumping(jumping) => {
                                                if self.y > y-jump_height && jumping {
                                                    self.y -= 5;
                                                } 
                                            
                                                if self.y < y && !jumping{
                                                    self.y += 5;
                                                } 
                                                
                                                if self.y <= y-jump_height {
                                                    self.jumping = Jump::Jumping(false);
                                                }
                                                if self.y >= y && !jumping {
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
