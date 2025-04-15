use super::*;
use sdl2::{EventPump, event::Event, keyboard::Scancode, render::Canvas, video::Window};

pub const WINDOW_WIDTH: usize = 160;
pub const WINDOW_HEIGHT: usize = 144;

pub struct UserInterface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub inputs: Inputs,
    pub running: bool,
}

pub struct Inputs {
    pub a: bool,
}

impl Inputs {
    fn new() -> Self {
        Inputs { a: false }
    }
}

impl UserInterface {
    pub fn new() -> Self {
        let (canvas, event_pump) = UserInterface::init_window();
        UserInterface {
            canvas,
            event_pump,
            inputs: Inputs::new(),
            running: true,
        }
    }

    fn init_window() -> (Canvas<Window>, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Gameboy", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        // Rendering
        let canvas = window.into_canvas().build().unwrap();

        // Window events
        let event_pump = sdl_context.event_pump().unwrap();

        (canvas, event_pump)
    }

    pub fn render_display() {}

    pub fn process_inputs(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => match scancode {
                    Scancode::A => self.inputs.a = true,
                    _ => {}
                },
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => match scancode {
                    Scancode::A => self.inputs.a = false,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
