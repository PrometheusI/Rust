use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

const PLAYER_SPEED_CONST: i32 = 2;




//Struct um die Richtung des spielers zu spiechern
#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction{
    //ändert den Input in eine Richtung um
    fn input_to_direction(direction: &DirectionInput) -> Direction{
        Direction{
            x: direction.right as i32 - direction.left as i32,
            y: direction.up as i32 - direction.down as i32,
        }
    }
}

//Struct um den Input zu speichern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectionInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl DirectionInput{
    //erstellt einen DirectionInput mit allen Feldern auf false
    fn default() -> DirectionInput {
        DirectionInput{
            up: false,
            down: false,
            right: false,
            left: false,
        }
    }
    //gibt aus ob eine Taste gedrückt wird
    fn anytrue(&self) -> bool{
        if self.down == true || self.up == true || self.left == true || self.right == true {
            return true;
        }
        false
    }
}

//Struct des Spielers
#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    directioninput: DirectionInput,
    current_frame: i32,
}

// gibt die Reihe in der die zur Richtung gehörenden Animation wieder 
fn direction_spritesheet_row(direction: &Direction) -> i32{
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

// Rendert den Hintergrund und Spieler
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
        player.sprite.y() + sprite_height as i32 * direction_spritesheet_row(&player.direction),
        sprite_width, sprite_height);
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position =  Point::new(width as i32 / 2 + player.position.x, height as i32 / 2 - player.position.y);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());


    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

// Updatet die Position des Spielers
fn update_player(player: &mut Player) {

   
    
    if player.directioninput.anytrue() {
        player.speed = PLAYER_SPEED_CONST;
        player.direction = Direction::input_to_direction(&player.directioninput);
    } else {player.speed = 0;}

    player.position = player.position.offset(player.speed * player.direction.x,
                                             player.speed * player.direction.y);

    if player.speed != 0 {
        player.current_frame = (player.current_frame +1) % 15;
    } else {
        player.current_frame = 5;
    }
}

//die main funktion gibt entweder ein leeren datentype wieder(()) oder einen String als Err(String)
fn main() -> Result<(), String> {
    // Erstellt ein Fenster mit den Maßen 800*600 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    //läd die Textur
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;


    //erstellt den Spieler
    let mut player = Player{
        direction: Direction { x: 1, y: 0 },
        directioninput: DirectionInput::default(),
        current_frame: 1,
        position: Point::new(0, 0),
        speed: 0,
        sprite: Rect::new(0,0,26,36),
    };

    //erstellt die Eventpump, die mein Input wahrnimmt
    let mut event_pump = sdl_context.event_pump()?;
    //erstellt eine Loop die als Lifetime running hat
    'running: loop {
        //nimmt die Events war und reagiert entsprechend. Kann mehrere Events pro frame wahrnehmen 
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.directioninput.left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.directioninput.right = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.directioninput.up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.directioninput.down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.directioninput.left = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.directioninput.right = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.directioninput.up = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.directioninput.down = false;
                },
                _ => {}
            }
        }

        // Updated den spieler
        update_player(&mut player);

        // Rendert den Spieler
        render(&mut canvas, Color::RGB(155, 155, 155), &texture, &player)?;

        // Sorgt dafür, dass es alle 1/30 Sekunden einen Frame gibt
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    //gibt den leeren datentypen zurück
    Ok(())
}