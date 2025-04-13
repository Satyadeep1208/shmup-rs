
use sdl2::pixels::Color;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::path::Path;


pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    // I believe this must be kept "underscored" to prevent
    // rust to drop it as dead code/unused variable;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    /*
    XXX
    although aspect ratio is 16:9, the game world should be 4:3,
    with the remaining space at each side of the screen used for
    HUD
    */

    /* XXX
    // probably set window size with the width and height
    // grabbed as demonstrated below, but probably only if
    // the values are smaller than the resolution set further
    // below in canvas.set_logical_size
    //
    // also, when needed, research the different behaviour of
    // `current_display_mode` and `desktop_display_mode` methods

    let dm = video_subsystem.current_display_mode(0)?;
    let (width, height) = (dm.w, dm.h);
    println!("width: {}, height: {}", width, height);

    // */

    let window = video_subsystem.window("Shmup", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    // XXX should we used a `software()` call (before build)?
    // would there be a problem if I didn't use it?
    // and what about `accelerated()` instead?
    // research;

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(960, 540).unwrap();

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
                    .load_texture(
                        Path::new("src/data/images/ship_0000.png")
                    )?;

    let mut trect = Rect::new(100, 100, texture.query().width, texture.query().height);

    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }

        }

        canvas.clear();

        canvas.copy(
            &texture,
            None,
            Some(trect),
        )?;

        canvas.present();

    }

    Ok(())

}
