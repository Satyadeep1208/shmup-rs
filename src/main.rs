extern crate sdl2;

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
    although screen is full HD, game world should be 1440:1080
    with remaining space at each side of the screen used for
    HUD
    */

    let window = video_subsystem.window("Shmup", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    canvas.set_logical_size(960, 540);

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
                    .load_texture(
                        Path::new("/home/kennedy/repos/shmup/ship_0000.png")
                    )?;

    let trect = Rect::new(100, 100, texture.query().width, texture.query().height);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.copy(
            &texture,
            None,
            Some(trect),
        );

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }

        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())

}
