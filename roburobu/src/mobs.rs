pub mod robu_robu;
pub mod obstacle;
use raylib::prelude::*;

pub trait Mob {
    fn update(&mut self);
    fn key_press(&mut self, rl: &RaylibHandle);
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}