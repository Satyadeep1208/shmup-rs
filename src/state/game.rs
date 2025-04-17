
use std::collections::HashSet;
use std::path::Path;
use std::ops::Not;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

use crate::state::loopholdertrait::LoopHolder;


struct Object<'a> {
    texture: Texture<'a>,
    rect: Rect,
}

pub struct Game<'a> {
    game_object: Object<'a>
}

impl<'a> Game<'a> {

    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {

        let texture = texture_creator.load_texture(
                        Path::new("src/data/images/ship_0000.png")
                      )?;

        let rect = Rect::new(
            100,
            100,
            texture.query().width,
            texture.query().height,
        );


        Ok(Self{ game_object: Object {texture, rect}})

    }
}


impl LoopHolder for Game<'_> {

    fn get_input(&mut self, event_pump: &mut EventPump) -> Result<(), String> {

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(String::from("quit"));
                },
                _ => {}
            }

        }

        let pressed: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();

        let (mut dx, mut dy) = (0, 0);

        if pressed.contains(&Scancode::W) {dy -= 1};
        if pressed.contains(&Scancode::A) {dx -= 1};
        if pressed.contains(&Scancode::S) {dy += 1};
        if pressed.contains(&Scancode::D) {dx += 1};

        if (dx == 0 && dy == 0).not() {
            dx *= 6;
            dy *= 6;
            self.game_object.rect.offset(dx, dy);
        }

        Ok(())

    }

    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        canvas.copy(
            &self.game_object.texture,
            None,
            Some(self.game_object.rect)
        )?;

        canvas.present();

        Ok(())

    }

}
