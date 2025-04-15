use crate::ppu::GbDisplay;

use super::*;
use sdl2::{
    EventPump, event::Event, keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

pub const WINDOW_WIDTH: usize = 256;
pub const WINDOW_HEIGHT: usize = 256;
pub const WINDOW_SCALE_FACTOR: usize = 1;

pub struct Inputs {
    pub a: bool,
}

impl Inputs {
    fn new() -> Self {
        Inputs { a: false }
    }
}

pub struct UserInterface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub inputs: Inputs,
    pub running: bool,
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

    pub fn render_display(&mut self, display: &GbDisplay) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for (y, row) in display.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let color = match pixel {
                    0 => Color::RGB(0, 0, 0),
                    1 => Color::RGB(255, 0, 0),
                    2 => Color::RGB(0, 255, 0),
                    3 => Color::RGB(0, 0, 255),
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
