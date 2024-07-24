use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH : i32 = 640;
const WINDOW_HEIGHT : i32 = 480;
const WINDOW_CENTER : Vector2 = Vector2::new((WINDOW_WIDTH/2) as f32, (WINDOW_HEIGHT/2) as f32);
const STARS_AMOUNT : u32 = 800;
const STAR_MAX_SIZE : f32 = 3.;
const MAX_SIMULATION_SPEED : f32 = 50.;


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Starfield")
        .build();
    rl.set_target_fps(60);

    let camera2D = Camera2D{target: Vector2::zero(), offset: WINDOW_CENTER, rotation : 0., zoom : 1.};

    let mut background_color : Color = Color{r:255, g: 255, b: 255, a: 255};
    let mut star_color : Color = Color{r : 0, g : 0, b : 0, a : 255};
    let mut simulation_speed : i32 = 10;

    let mut stars : Vec<Star> = Vec::new();
    for _ in 0..STARS_AMOUNT{
        stars.push(Star::new());
    }
    while !rl.window_should_close() {


        let mouse_pos_vec = rl.get_mouse_position();

        // Calculating the simulation speed depending on mouse X position within the window frame, 
        // where higher mouse x coordinates give faster speed 
        simulation_speed = (mouse_pos_vec.x / WINDOW_WIDTH as f32 * MAX_SIMULATION_SPEED) as i32;


        // Calculating the background color rgb channels value depending on mouse Y position within the window frame,
        // where higher mouse Y coordinates give brighter overall color 
        let bg_color_channels : u8 =  (mouse_pos_vec.y as f32 / WINDOW_HEIGHT as f32 * 255.) as u8;
        background_color = Color{r: bg_color_channels, g : bg_color_channels, b : bg_color_channels, a : 255};

        // Inverting background rgb channels value for stars in order to create a contrast and to be able to see stars
        star_color = Color{r: 255 - bg_color_channels, g : 255 - bg_color_channels, b : 255 - bg_color_channels, a : 255};

        let mut d = rl.begin_drawing(&thread);


        d.clear_background(background_color);
        {

            // Starting in 2d camera mode in order to shift
            // the window coordinate origin to the screen center
            let mut d = d.begin_mode2D(camera2D);

            for star in stars.iter_mut(){
                star.update(simulation_speed);
                d.draw_circle(star.x_speed, star.y_speed, star.radius, star_color);
                d.draw_line(star.x_prev, star.y_prev, star.x_speed, star.y_speed, star_color);
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

    pub fn update(&mut self, simulation_speed : i32){
        self.z -= simulation_speed;
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