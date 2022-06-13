use raylib::prelude::*;
use crate::utils::Timer;

use crate::mobs::obstacle::Obstacle;

pub fn render() {
    
    let (mut rl, thread) = raylib::init()
                        .size(840, 480)
                        .title("Robu Robu")
                        .build();

    rl.set_target_fps(60);

    let mut robu_robu = crate::mobs::robu_robu::RobuRobu::create(240, 298, &mut rl, &thread);
    let mut obstacles: Vec<Obstacle> = vec![Obstacle::create(640, 350, &mut rl)];
    let mut obstacle_spawn_time = Timer::create();
    let mut score_timer = Timer::create_fixed(100);
    let mut score: i128 = 0;

    while !rl.window_should_close() {

        if obstacle_spawn_time.timer_fired() {
            obstacles.push(Obstacle::create(840, 350, &mut rl))
        }

        //detect keypresses
        robu_robu.key_press(&rl);

        //update mobs
        robu_robu.update();
        
        for i in obstacles.iter_mut() {
            i.update(score);
            if i.get_rect().check_collision_recs(&robu_robu.get_rect()) {
                return
            }
        }


        let mut d = rl.begin_drawing(&thread);


        d.clear_background(Color::WHITE);

        d.draw_text(score.to_string().as_str(), 0, 0, 60, Color::BLACK);

        if score_timer.timer_fired() { score+=1 };

        crate::draw(&mut d);
        robu_robu.draw(&mut d);
  
        for i in obstacles.iter_mut() {
            i.draw(&mut d);
        }
    }

}