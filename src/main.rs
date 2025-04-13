
mod sdlsetup;

use std::time::Duration;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use sdlsetup::setup;


pub fn main() -> Result<(), String> {

    let mut sdl_refs = setup().unwrap();

    let texture = sdl_refs.texture_creator
                    .load_texture(
                        Path::new("src/data/images/ship_0000.png")
                    )?;

    let mut trect = Rect::new(
        100,
        100,
        texture.query().width,
        texture.query().height,
    );

    sdl_refs.canvas.set_draw_color(Color::RGB(100, 100, 100));
    sdl_refs.canvas.clear();
    sdl_refs.canvas.present();

    'running: loop {

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        for event in sdl_refs.event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }

        }

        sdl_refs.canvas.clear();

        sdl_refs.canvas.copy(
            &texture,
            None,
            Some(trect),
        )?;

        sdl_refs.canvas.present();

    }

    Ok(())

}
