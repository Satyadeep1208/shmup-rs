
use sdl2::EventPump;
use sdl2::render::{WindowCanvas};


pub trait LoopHolder {

    fn get_input(&mut self, event_pump: &mut EventPump) -> Result<(), String>;
    fn update(&mut self) -> Result<(), String> {Ok(())}
    fn draw(&self, canvas: &mut WindowCanvas)-> Result<(), String>;

    fn apploop(&mut self, event_pump: &mut EventPump, canvas: &mut WindowCanvas) -> Result<(), String> {

        self.get_input(event_pump)?;
        self.update()?;
        self.draw(canvas)?;

        Ok(())

    }
}
