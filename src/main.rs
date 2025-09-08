use raylib::prelude::*;
use raylib::consts::*;
use rand::Rng;

//Window sizes
const SW : f64 = 640.0;
const SH : f64 = 600.0;
const SW2 : f64 = SW/2.0;
const SH2 : f64 = SH/2.0;
const GRAVITY : f64 = 9.81 * 2.0;

//PIPES consts values
const PIPE_W : f64 = 90.0;
const PIPE_H : f64 = 550.0;
const SPACE_BETWEEN_PIPES : f64 = 200.0;
const MAX_PIPES : usize = 4;

//Vector2 definition
#[derive(Clone, Copy)]
struct Vector2{
    x : f64,
    y : f64
}

impl Vector2 {
    fn print(&self) { println!("( {}, {} )",self.x,self.y) }
    fn new(x : f64, y : f64) -> Self{ Self { x, y } }
    fn zero() -> Vector2 { Vector2 { x : 0.0, y : 0.0 } }
}

//Collision shapes
#[derive(Clone, Copy)]
struct Rect {
    position : Vector2,
    width : f64,
    height : f64,
}

impl Rect {

    fn new(position : Vector2,width : f64, height : f64) -> Self{
        Rect { position, width, height }
    }

    fn draw(&self,d : &mut RaylibDrawHandle){
        d.draw_rectangle(self.position.x as i32, self.position.y as i32, self.width as i32, self.height as i32, Color::BLUE);
    }

    fn intersects_aabb(A : &Rect, B : &Rect) -> bool {
        let collisionX = A.position.x + A.width >= B.position.x && B.position.x + B.width >= A.position.x;
        let collisionY = A.position.y + A.height >= B.position.y && B.position.y + B.height >= A.position.y;
        collisionX && collisionY
    }
}



/*Player definition*/
struct Player{
    position : Vector2,
    velocity : Vector2,
    sprite : Texture2D
}

impl Player{

    fn new(rl : &mut RaylibHandle, thread : &RaylibThread ) -> Self{
        Self {
            position : Vector2::new( SW2, SH2),
            velocity : Vector2::zero(),
            sprite : rl.load_texture(thread,"Assets/player.png").expect("Could not load player texture")
        }
    }

    fn reset(&mut self) {
        self.position = Vector2::new( SW2 - self.sprite.width() as f64 * 0.5, SH2 - self.sprite.height() as f64 * 0.5 );
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
        //d.draw_circle(self.position.x as i32,self.position.y as i32,30.0,Color::RED);
        d.draw_texture(
            &self.sprite,
            self.position.x as i32,
            self.position.y as i32,
            Color::WHITE
            );
    }

}

//Pipe managers
#[derive(Clone,Copy)]
struct Pipe{
    top : Rect,
    bottom : Rect,
    flag : bool,
}

impl Pipe{

    fn new(top : Rect, bottom : Rect) -> Self {
        Pipe{
            top,
            bottom,
            flag : true
        }
    }

    fn out_of_bounds(&self) -> bool {
        self.top.position.x <= - PIPE_W - 10.0
    }

    fn reset_pipe_x(&mut self){
        let posx : f64 = SW + (PIPE_W * (MAX_PIPES as f64 + 1.5) ) ;
        self.top.position.x = posx;
        self.bottom.position.x = posx;
    }

    fn player_has_passed(&self, player : &Player) -> bool{
        player.position.x >= self.top.position.x && self.flag
    }

    fn reset_pipe_y(&mut self){
        let pivot : f64 = ( rand::thread_rng().gen_range(100..SH as i32 -150) ) as f64;
        self.top.position.y = pivot - (SPACE_BETWEEN_PIPES*0.5) - PIPE_H;
        self.bottom.position.y = pivot + (SPACE_BETWEEN_PIPES*0.5);
    }
}

struct PipeManager{
    pipes : [Pipe;MAX_PIPES],
    top_sprite : Texture2D,
    bottom_sprite : Texture2D
}

impl PipeManager{
    fn new(rl : &mut RaylibHandle, thread : &RaylibThread) -> Self {
        let rect1 : Rect = Rect::new(Vector2::zero(),PIPE_W,PIPE_H);
        let rect2 : Rect = Rect::new(Vector2::zero(),PIPE_W,PIPE_H);
        let pipe : Pipe = Pipe::new(rect1,rect2);
        PipeManager { 
            pipes : [ pipe ; MAX_PIPES],
            top_sprite : rl.load_texture(thread,"Assets/top_pipe.png").expect("Could not load top pipe sprite"),
            bottom_sprite : rl.load_texture(thread,"Assets/bottom_pipe.png").expect("Could not load bottom sprite")
        }
    }

    fn reset(&mut self){
        for i in 0..MAX_PIPES {
            let dist_x : f64 = PIPE_W * 3.5;
            let start_pos_x : f64 = i as f64 * dist_x;
            self.pipes[i].reset_pipe_y();
            self.pipes[i].flag = true;
            self.pipes[i].top.position.x = SW + 50.0 + start_pos_x;
            self.pipes[i].bottom.position.x = SW + 50.0 + start_pos_x;
        }
    }

    fn player_collision_pipes(&self,player : &Player) -> bool{
        let player_rect : Rect = Rect::new(Vector2::new(player.position.x,player.position.y),player.sprite.width() as f64,player.sprite.height() as f64);
        let floor_rect : Rect = Rect::new(Vector2::new(0.0,SH-20.0),SW,130.0);

        if Rect::intersects_aabb(&player_rect,&floor_rect) {
            return true;
        }

        for i in 0..MAX_PIPES{
            if Rect::intersects_aabb(&player_rect,&self.pipes[i].top) ||Rect::intersects_aabb(&player_rect, &self.pipes[i].bottom) {
                return true;
            }
        }
        return false;
    }

    fn update(&mut self, dt : f64) {
        let speed_movement : f64 = 200.0;
        for i in 0..MAX_PIPES {
            self.pipes[i].top.position.x -= speed_movement * dt;
            self.pipes[i].bottom.position.x -= speed_movement * dt;
            if self.pipes[i].out_of_bounds() {
                self.pipes[i].reset_pipe_x();
                self.pipes[i].reset_pipe_y();
                self.pipes[i].flag = true;
            }
        }

    }

    fn render(&self, d : &mut RaylibDrawHandle){
        for i in 0..MAX_PIPES {
            //self.pipes[i].top.draw(d);
            d.draw_texture(
                &self.top_sprite,
                self.pipes[i].top.position.x as i32,
                self.pipes[i].top.position.y as i32,
                Color::WHITE
                );
            //self.pipes[i].bottom.draw(d);
            d.draw_texture(
                &self.bottom_sprite,
                self.pipes[i].bottom.position.x as i32,
                self.pipes[i].bottom.position.y as i32,
                Color::WHITE
                );
        }
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
    let mut player : Player = Player::new(rl,thread);
    let mut pipes : PipeManager = PipeManager::new(rl,thread);
    let mut score : i32 = 0;
    let mut background_sprites : [Texture2D ; 2] = [
        rl.load_texture(thread,"Assets/background.png").expect("Could not load background sprite"),
        rl.load_texture(thread,"Assets/grass.png").expect("Could not load background sprite")
    ];
    player.reset();
    pipes.reset();
    while !rl.window_should_close() {
        let delta_time : f64 = rl.get_frame_time() as f64;
        update(rl,delta_time,&mut player,&mut pipes,&mut score);
        render(rl,thread,&player,&pipes,&mut background_sprites,score);
    }
}

fn update_score(score : &mut i32,player : &Player,pipes : &mut PipeManager){
    for i in 0..MAX_PIPES{
        if pipes.pipes[i].player_has_passed(player){
            *score += 1;
            pipes.pipes[i].flag = false;
        }
    }
}

fn update(rl : &mut RaylibHandle, dt : f64 ,player : &mut Player,pipes : &mut PipeManager, score : &mut i32){
    player.update(rl,dt);
    pipes.update(dt);
    update_score(score,player,pipes);
    if pipes.player_collision_pipes(&player) {
        player.reset();
        pipes.reset();
        *score = 0;
    }
}

fn render(rl : &mut RaylibHandle,thread : &RaylibThread,player : &Player, pipes : &PipeManager, background : &mut [Texture2D],score : i32){
    let mut d : RaylibDrawHandle = rl.begin_drawing(thread);//Lets us acces drawing calls
    let score_text : &str =  &score.to_string()[..];
    d.clear_background( Color::WHITE);
    d.draw_texture(&background[0],0,0,Color::WHITE);
    pipes.render(&mut d);
    d.draw_texture(&background[1],0,0,Color::WHITE);
    player.render(&mut d);
    d.draw_text(score_text,SW2 as i32 -10,10,80,Color::WHITE);
    d.draw_text(score_text,SW2 as i32 -5,5,80,Color::BLACK);
}

