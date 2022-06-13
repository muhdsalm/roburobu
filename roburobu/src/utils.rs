use std::time::Instant;

use rand::Rng;

pub struct Timer { 

    obstacle_spawn_interval: u128,
    then_time: Instant,
    fixed: bool,

}

impl Timer {
    fn gen_obstacle_spawn_intervel(&mut self) {

        self.obstacle_spawn_interval = rand::thread_rng().gen_range(800..3000);
        self.then_time = Instant::now();
        println!("{}", self.obstacle_spawn_interval)
    }

    pub fn timer_fired(&mut self) -> bool {
        if self.obstacle_spawn_interval == 0 {
            self.gen_obstacle_spawn_intervel();
        }
        if self.then_time.elapsed().as_millis() > self.obstacle_spawn_interval {

            if self.fixed {
                self.then_time = Instant::now();
                return true;
            }

            self.gen_obstacle_spawn_intervel();
            return true
        }
        false
    }

    pub fn create() -> Self {
        Self{then_time: Instant::now(), obstacle_spawn_interval: 0, fixed: false}
    }
    pub fn create_fixed(time: u128) -> Self {
        Self{then_time: Instant::now(), obstacle_spawn_interval: time, fixed: true}
    }
}