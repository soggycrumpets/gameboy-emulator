use std::iter::Scan;

use crate::ppu::GbDisplay;

use super::*;
use sdl2::{
    EventPump, event::Event, keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

pub const WINDOW_WIDTH: usize = 256;
pub const WINDOW_HEIGHT: usize = 256;
pub const WINDOW_SCALE_FACTOR: usize = 1;

#[derive(Copy, Clone, Debug)]
pub struct Inputs {
    pub a: bool,
    pub g: bool,
    pub m: bool,
    pub n: bool,
    pub r: bool,
    pub p: bool,
}

impl Inputs {
    fn new() -> Self {
        Inputs {
            a: false,
            g: false,
            m: false,
            n: false,
            r: false,
            p: false,
        }
    }

    // Returns false if the key has not been implemented
    fn get(&self, scancode: Scancode) -> bool {
        match scancode {
            Scancode::A => self.a,
            Scancode::G => self.g,
            Scancode::M => self.m,
            Scancode::N => self.n,
            Scancode::R => self.r,
            Scancode::P => self.p,
            _ => false,
        }
    }

    fn set(&mut self, scancode: Scancode, set: bool) {
        match scancode {
            Scancode::A => self.a = set,
            Scancode::G => self.g = set,
            Scancode::M => self.m = set,
            Scancode::N => self.n = set,
            Scancode::R => self.r = set,
            Scancode::P => self.p = set,
            _ => (),
        };
    }
}

pub struct UserInterface {
    pub inputs_down: Inputs,
    inputs_was_down: Inputs,
    pub inputs_unique: Inputs,

    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub running: bool,
}

impl UserInterface {
    pub fn new() -> Self {
        let (canvas, event_pump) = UserInterface::init_window();
        UserInterface {
            canvas,
            event_pump,
            inputs_down: Inputs::new(),
            inputs_was_down: Inputs::new(),
            inputs_unique: Inputs::new(),
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

    pub fn render_display(&mut self, display: &GbDisplay) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for (y, row) in display.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let color = match pixel {
                    0 => Color::RGB(8, 24, 32),
                    1 => Color::RGB(52, 104, 86),
                    2 => Color::RGB(136, 192, 112),
                    3 => Color::RGB(224, 248, 208),
                    _ => panic!("Invalid pixel color value detected in the display"),
                };
                self.canvas.set_draw_color(color);

                self.canvas
                    .fill_rect(Rect::new(
                        (x as i32) * (WINDOW_SCALE_FACTOR as i32),
                        (y as i32) * (WINDOW_SCALE_FACTOR as i32),
                        WINDOW_SCALE_FACTOR as u32,
                        WINDOW_SCALE_FACTOR as u32,
                    ))
                    .unwrap();
            }
        }

        self.canvas.present();
    }

    pub fn process_inputs(&mut self) {
        // Update previous inputs
        self.inputs_was_down = self.inputs_down;

        // Update inputs that are currently down
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => self.inputs_down.set(scancode, true),
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => self.inputs_down.set(scancode, false),
                _ => {}
            }
        }

        // Update unique inputs
        // TODO: Inplement a generic unique input checking system (Loop through all scancodes)
        for scancode in [Scancode::A, Scancode::G, Scancode::M, Scancode::N, Scancode::R, Scancode::P] {
            let a_down = self.inputs_down.get(scancode);
            let a_was_down = self.inputs_was_down.get(scancode);
            let a_unique = a_down && !a_was_down;
            self.inputs_unique.set(scancode, a_unique);
        }
    }
}

mod input {}
