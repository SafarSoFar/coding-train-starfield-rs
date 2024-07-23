use std::{ops::{Add, Div, Sub, SubAssign}, thread::sleep, time::Duration};
use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH : i32 = 640;
const WINDOW_HEIGHT : i32 = 480;
const WINDOW_CENTER : Vector2 = Vector2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32);
const STARS_AMOUNT : u32 = 800;
const STAR_MAX_SIZE : f32 = 12.;
const SIMULATION_SPEED : i32 = 50;


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
                //d.draw_circle(star.x_speed, star.y_speed, star.radius, Color::BLACK);
                d.draw_line(star.x_prev, star.y_prev, star.x_speed, star.y_speed, Color::BLACK);
            }

        }


    }
}

struct Star{
    x : i32,
    y : i32,
    z : i32,

    x_speed : i32,
    y_speed : i32,

    x_prev : i32,
    y_prev : i32,
    z_prev : i32,


    radius : f32,
    
}

impl Star{
    pub fn new() -> Self{
        Self {
            x : thread_rng().gen_range(-WINDOW_WIDTH..WINDOW_WIDTH), 
            y: thread_rng().gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT), 
            z: thread_rng().gen_range(0..WINDOW_WIDTH), 

            x_speed: 0,
            y_speed : 0,

            x_prev : 0,
            y_prev : 0,
            z_prev : 0,

            radius : 0.,

        }
    }

    pub fn change_pos(&mut self){
        self.z -= SIMULATION_SPEED;
        if self.z < 1{
            self.z = WINDOW_WIDTH;
            self.x = thread_rng().gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
            self.y = thread_rng().gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);

            self.z_prev = self.z;

        }

        // mapping radius from 16 to 0 depending on how far the star is;
        self.radius = STAR_MAX_SIZE - self.z as f32 * STAR_MAX_SIZE /WINDOW_WIDTH as f32;

        self.x_speed = map_star_speed(self.x as f32 / self.z as f32, WINDOW_WIDTH as f32);
        self.y_speed = map_star_speed(self.y as f32 / self.z as f32, WINDOW_HEIGHT as f32);

        self.x_prev = map_star_speed(self.x as f32 / self.z_prev as f32, WINDOW_WIDTH as f32);
        self.y_prev = map_star_speed(self.y as f32 / self.z_prev as f32, WINDOW_HEIGHT as f32);
        self.z_prev = self.z;
        
    }
}

pub fn map_star_speed(star_screen_ratio : f32, boundary : f32) -> i32{
    // (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    (star_screen_ratio * boundary) as i32
}