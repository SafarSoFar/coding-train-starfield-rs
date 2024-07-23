use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH : i32 = 640;
const WINDOW_HEIGHT : i32 = 480;



fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Starfield")
        .build();
    
    let x = rl.get_random_value::<i32>(0..WINDOW_WIDTH);
    let y = rl.get_random_value::<i32>(0..WINDOW_HEIGHT);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_circle(x,y, 5.0, Color::BLACK);
        d.clear_background(Color::WHITE);
        //d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

struct Star{
    x : u32,
    y : u32,
    z : i32,
}