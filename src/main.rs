use std::{ops::{Add, Div, Sub, SubAssign}, thread::sleep, time::Duration};
use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH : i32 = 640;
const WINDOW_HEIGHT : i32 = 480;
const WINDOW_CENTER : Vector2 = Vector2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32);
const STARS_AMOUNT : u32 = 100;


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
            let mut d = d.begin_mode2D(camera2D);
            for star in stars.iter_mut(){
                star.change_pos();
                d.draw_circle(star.x_cur, star.y_cur, 5., Color::BLACK);
            }
        }//rl.mode2d


    }
}

struct Star{
    x_init : i32,
    y_init : i32,
    z_init : i32,
    x_cur : i32,
    y_cur : i32,
}

impl Star{
    pub fn new() -> Self{
        Self {
            x_init : thread_rng().gen_range(-WINDOW_WIDTH..WINDOW_WIDTH), 
            y_init: thread_rng().gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT), 
            z_init: WINDOW_WIDTH, 
            x_cur: 0,
            y_cur : 0,
        }
    }

    pub fn change_pos(&mut self){
        self.z_init -= 1;
        //self.x -= 1;
        //self.y -= 1;
        self.x_cur = map_star_speed(self.x_init as f32 / self.z_init as f32, WINDOW_WIDTH as f32);
        self.y_cur = map_star_speed(self.y_init as f32 / self.z_init as f32, WINDOW_HEIGHT as f32);
    }
}

pub fn map_star_speed(star_screen_ratio : f32, boundary : f32) -> i32{
    (star_screen_ratio * boundary) as i32
}