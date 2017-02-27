extern crate boxcrash;

use boxcrash::game::{Game, GameConfig};
use boxcrash::Pixel;
use std::f64::consts::PI;

fn main() {
    let config = GameConfig {
        title: "Box Crash",
        screen_size: Pixel::new(1366, 768),
        ups: 60,
        max_fps: 60,
        tunel_size: [15., 8., 150.],
        player_size: [1.5, 0.8, 3.],
        player_speed: (20., 120.),
        player_turn_speed: 15.,
        bot_size: [(1., 4.), (0.5, 2.5), (2.5, 8.)],
        bot_speed: (20., 120.),
        bot_turn_speed: (5., 20.),
        divider_size: [1., 7.],
        camera_height: 3.,
        camera_distance: 5.5,
        decor_distance: 8.,
        sprint_factor: 15.,
        spawn_time: (0.25, 1.),
        game_sprint: 1.,
        game_max_speed: 80.,
        player_jump_v: 7.,
        player_jump_a: 5.,
        jump_turn_decrease: 3.,
        jump_timeout: 8.,
        mouse_speed: PI/420.,
        trueshot_distance: 100.,
        bullet_stock: 15,
        recharge_time: 10.,
        bullet_len: 5.,
        bullet_speed: 100.,
        zoom_in: false,
    };
    Game::new(config).run();
}
