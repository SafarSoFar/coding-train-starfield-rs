use std::{ops::{Add, Div, Sub, SubAssign}, thread::sleep, time::Duration};
use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH : i32 = 640;
const WINDOW_HEIGHT : i32 = 480;
const WINDOW_CENTER : Vector2 = Vector2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32);
const STARS_AMOUNT : u32 = 600;
const STAR_MAX_SIZE : f32 = 12.;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Starfield")
        .build();
    rl.set_target_fps(60);

    let camera2D = Camera2D{target: Vector2::zero(), offset: WINDOW_CENTER, rotation : 0., zoom : 1.};

    let mut stars : Vec<Star> = Vec::new();
    for _ in 0..STARS_AMOUNT{
        stars.push(Star::new());
    }
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        {

            // Starting in 2d camera mode in order to shift
            // the window coordinate origin to the screen center
            let mut d = d.begin_mode2D(camera2D);

            for star in stars.iter_mut(){
                star.change_pos();
                d.draw_circle(star.x_cur, star.y_cur, star.radius, Color::BLACK);
            }

        }


    }
}

struct Star{
    x_init : i32,
    y_init : i32,
    z_init : i32,
    x_cur : i32,
    y_cur : i32,
    radius : f32,
}

impl Star{
    pub fn new() -> Self{
        Self {
            x_init : thread_rng().gen_range(-WINDOW_WIDTH..WINDOW_WIDTH), 
            y_init: thread_rng().gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT), 
            z_init: thread_rng().gen_range(0..WINDOW_WIDTH), 
            x_cur: 0,
            y_cur : 0,
            radius : 0.,
        }
    }

    pub fn change_pos(&mut self){
        self.z_init -= 10;
        if self.z_init < 1{
            self.z_init = WINDOW_WIDTH;
            self.x_init = thread_rng().gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
            self.y_init = thread_rng().gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
        }

        // mapping radius from 16 to 0 depending on how far the star is;
        self.radius = STAR_MAX_SIZE - self.z_init as f32 * STAR_MAX_SIZE /WINDOW_WIDTH as f32;

        self.x_cur = map_star_speed(self.x_init as f32 / self.z_init as f32, WINDOW_WIDTH as f32);
        self.y_cur = map_star_speed(self.y_init as f32 / self.z_init as f32, WINDOW_HEIGHT as f32);
    }
}

pub fn map_star_speed(star_screen_ratio : f32, boundary : f32) -> i32{
    // (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    (star_screen_ratio * boundary) as i32
}