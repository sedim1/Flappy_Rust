use raylib::prelude::*;
use raylib::consts::*;

//Window sizes
const SW : f64 = 640.0;
const SH : f64 = 800.0;
const SW2 : f64 = SW/2.0;
const SH2 : f64 = SH/2.0;
const GRAVITY : f64 = 9.81 * 2.0;

//Vector2 definition
#[derive(Debug)]
struct Vector2{
    x : f64,
    y : f64
}

impl Vector2 {
    fn print(&self) { println!("( {}, {} )",self.x,self.y) }
    fn new(x : f64, y : f64) -> Self{ Self { x, y } }
    fn zero() -> Vector2 { Vector2 { x : 0.0, y : 0.0 } }
}

/*Player definition*/
struct Player{
    position : Vector2,
    velocity : Vector2,
}

impl Player{

    fn new() -> Self{
        Self {
            position : Vector2::new( SW2, SH2),
            velocity : Vector2::zero()
        }
    }

    fn reset_player(&mut self) {
        self.position = Vector2::new( SW2, SH2 );
        self.velocity = Vector2::zero();
    }

    fn update(&mut self, rl : &RaylibHandle ,dt : f64){
        const JUMPHEIGHT : f64 = -7.0;
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.velocity.y = JUMPHEIGHT;
        }
        else {
            self.velocity.y = self.velocity.y + dt * GRAVITY;
        }

        self.position.y = self.position.y + self.velocity.y

    }

    fn render(&self,d : &mut RaylibDrawHandle){
        d.draw_circle(self.position.x as i32,self.position.y as i32,30.0,Color::RED);
    }

}


fn main() {
    //Init raylib and setup window
    let (mut rl, thread) : (RaylibHandle,RaylibThread) = raylib::init()
        .size(SW as i32,SH as i32)
        .title("FlappyBird")
        .build();
    rl.set_window_min_size(SW as i32,SH as i32);
    rl.set_window_max_size(SW as i32,SH as i32);
    rl.set_target_fps(60);
    
    main_loop(&mut rl,&thread);
}

fn main_loop(rl : &mut RaylibHandle, thread : &RaylibThread) {
    let mut player = Player::new();

    while !rl.window_should_close() {
        let delta_time : f64 = rl.get_frame_time() as f64;
        update(rl,delta_time,&mut player);
        render(rl,thread,&player);
    }
}

fn update(rl : &mut RaylibHandle, dt : f64 ,player : &mut Player){
    player.update(rl,dt);
}

fn render(rl : &mut RaylibHandle,thread : &RaylibThread,player : &Player){
    let mut d : RaylibDrawHandle = rl.begin_drawing(thread);//Lets us acces drawing calls
    d.clear_background( Color::WHITE);
    player.render(&mut d);
}

