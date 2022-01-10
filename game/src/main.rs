use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::Builder;
use specs::prelude::{Component, VecStorage, World, WorldExt};
use specs_derive::Component;

use std::time::Duration;

const PLAYER_SPEED_CONST: i32 = 2;



#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Sprite {
    /// The specific spritesheet to render from
    spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    current_frame: usize,
    up_frames: Vec<Sprite>,
    down_frames: Vec<Sprite>,
    right_frames: Vec<Sprite>,
    left_frames: Vec<Sprite>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Direction {
    x: i32,
    y: i32,
}
impl Direction{
    fn input_to_Direction(direction: &DirectionInput) -> Direction{
        Direction{
            x: direction.right as i32 - direction.left as i32,
            y: direction.up as i32 - direction.down as i32,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
#[storage(VecStorage)]
struct DirectionInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl DirectionInput{
    fn default() -> DirectionInput {
        DirectionInput{
            up: false,
            down: false,
            right: false,
            left: false,
        }
    }

    fn anytrue(&self) -> bool{
        if self.down == true || self.up == true || self.left == true || self.right == true {
            return true;
        }
        false
    }
}

// Returns the coroosponding row the given direction 
fn direction_spritesheet_row(direction: Direction) -> i32{
    if direction.x == 1 {
        return 2;
    }
    if direction.x == -1 {
        return 1;
    }
    if direction.y == 1{
        return 3;
    }
    0
}

fn character_animations_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite>{
    // All assumptions about the spritesheets are now encapsulated in this function instead of in
    // the design of our entire system. We can always replace this function, but replacing the
    // entire system is harder.

    let (frame_width , frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();

    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height, 
            )
        })
    }

    frames
}


// not updated
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let (sprite_width, sprite_height) = player.sprite.size();

    let current_frame = Rect::new(
        player.sprite.x() + sprite_width as i32 * (player.current_frame/5),
        player.sprite.y() + sprite_height as i32 * player.direction.direction_4(),
        sprite_width, sprite_height);
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position =  Point::new(width as i32 / 2 + player.position.x, height as i32 / 2 - player.position.y);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());


    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

// Update player a fixed amount based on their speed.
// WARNING: Calling this function too often or at a variable speed will cause the player's speed
// to be unpredictable!
fn update_player(player: &mut Player) {

   
    
    if player.DirectionInput.anytrue() {
        player.speed = PLAYER_SPEED_CONST;
        player.direction = Direction::input_to_Direction(&player.DirectionInput);
    } else {player.speed = 0;}

    player.position = player.position.offset(player.speed * player.direction.x,
                                             player.speed * player.direction.y);

    if player.speed != 0 {
        player.current_frame = (player.current_frame +1) % 15;
    } else {
        player.current_frame = 1;
    }
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];

    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 1,
        up_frames: character_animations_frames(player_spritesheet, player_top_left_frame, Direction{x:0 ,y:1}),
        down_frames: character_animations_frames(player_spritesheet, player_top_left_frame, Direction{x:0 ,y:-1}),
        right_frames: character_animations_frames(player_spritesheet, player_top_left_frame, Direction{x:1 ,y:0}),
        left_frames: character_animations_frames(player_spritesheet, player_top_left_frame, Direction{x:-1 ,y:0}),
    };

    let mut world = World::new();

    world.create_entity()
        .with(DirectionInput::default())
        .with(Position(Point::new(0,0)))
        .with(Velocity{x: 0, y: 0})
        .with(player_animation.down_frames[0].clone())
        .with(player_animation)
        .build();

    world.maintain();

    let mut event_pump = sdl_context.event_pump()?;
    //let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.DirectionInput.left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.DirectionInput.right = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.DirectionInput.up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.DirectionInput.down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.DirectionInput.left = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.DirectionInput.right = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.DirectionInput.up = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.DirectionInput.down = false;
                },
                _ => {}
            }
        }

        // Update
        //i = (i + 1) % 255;
        update_player(&mut player);

        // Render
        render(&mut canvas, Color::RGB(155, 155, 155), &texture, &player)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}