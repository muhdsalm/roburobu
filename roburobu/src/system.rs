use raylib::prelude::*;
use crate::mobs::robu_robu::RobuRobu;
use crate::utils::Timer;

use crate::mobs::obstacle::Obstacle;

#[derive(PartialEq)]
enum GameState {
    Running,
    Dead
}

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
    let mut delete_something = false;
    let mut what_to_delete: Obstacle = Obstacle::create(0, 0, &mut rl);
    let mut game_state = GameState::Running;

    while !rl.window_should_close() {

        

        if game_state == GameState::Running {
            running_updates(&mut obstacle_spawn_time, &mut robu_robu, &mut obstacles, &mut rl, &mut score, &mut delete_something, &mut what_to_delete, &mut game_state, &mut score_timer)
        } else if game_state == GameState::Dead {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                robu_robu = crate::mobs::robu_robu::RobuRobu::create(240, 298, &mut rl, &thread);
                obstacles = vec![Obstacle::create(640, 350, &mut rl)];
                obstacle_spawn_time = Timer::create();
                score_timer = Timer::create_fixed(100);
                score = 0;
                delete_something = false;
                what_to_delete = Obstacle::create(0, 0, &mut rl);
                game_state = GameState::Running;
            }
        }

        let mut d = rl.begin_drawing(&thread);

        if game_state == GameState::Running {
            running_draws(&mut d, &mut score, &mut robu_robu, &mut obstacles);
        } else if game_state == GameState::Dead {
            running_draws(&mut d, &mut score, &mut robu_robu, &mut obstacles);
            d.draw_rectangle(0, 0, 840, 480, Color::new(255, 255, 255, 100));
            d.draw_text("GAME OVER! \nPRESS SPACE TO RESTART", 30, 70, 50, Color::LIGHTGRAY);
        }


        
    }

}

fn running_draws(d: &mut RaylibDrawHandle, score: &mut i128, robu_robu: &mut RobuRobu, obstacles: &mut Vec<Obstacle>) {
        d.clear_background(Color::DARKGRAY);

        d.draw_text(score.to_string().as_str(), 0, 0, 60, Color::BLACK);


        crate::draw(d);
        robu_robu.draw(d);
  
        for i in obstacles.iter_mut() {
            i.draw(d);
        }
}

fn running_updates(obstacle_spawn_time: &mut Timer, robu_robu: &mut RobuRobu, obstacles: &mut Vec<Obstacle>, rl: &mut RaylibHandle, score: &mut i128, delete_something: &mut bool, what_to_delete: &mut Obstacle, game_state: &mut GameState, score_timer: &mut Timer) {
    if obstacle_spawn_time.timer_fired(*score) {
            obstacles.push(Obstacle::create(840, 350, rl))
        }

        if score_timer.timer_fired(*score) { *score+=1 };

        //detect keypresses
        robu_robu.key_press(&rl);

        //update mobs
        robu_robu.update();
        
        for i in obstacles.iter_mut() {
            i.update(*score);
            if i.get_rect().check_collision_recs(&robu_robu.get_rect()) {
                *game_state = GameState::Dead;
            }
            if i.get_x() < -25 {
                *delete_something = true;
                *what_to_delete = *i;
            }

            
        }

        if *delete_something {
            obstacles.retain(|&x| x != *what_to_delete);
        }
}