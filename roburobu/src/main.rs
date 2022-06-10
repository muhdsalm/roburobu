mod system;
mod mobs;
mod utils;
use raylib::prelude::*;

fn main() {

    system::render();
}

fn draw(d: &mut RaylibDrawHandle) {

    d.draw_rectangle(0, 400, 840, 480, Color::BLACK);
        
    //d.draw_rectangle(240, 320, 50, 80, Color::BLACK);

}

