//! Example from SFML: Shape

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, CustomShape, ShapeImpl, RenderTarget};
use sfml::window::{VideoMode, ContextSettings, event, window_style, Key};
use sfml::system::Vector2f;

#[derive(Clone, Copy)]
pub struct CustomShapeImpl;

impl ShapeImpl for CustomShapeImpl {
    fn get_point_count(&self) -> u32 {
        3
    }

    fn get_point(&self, point: u32) -> Vector2f {
        match point {
            0 => Vector2f {x: 10., y: 10.},
            1 => Vector2f {x: 100., y: 100.},
            2 => Vector2f {x: 200., y: 300.},
            _ => panic!("error")
        }
    }
}

fn main() {
    // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Shape Example", window_style::CLOSE, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);


    let mut shape = CustomShape::new(Box::new(CustomShapeImpl)).expect("Error, cannot create a Shape");
    shape.set_fill_color(&Color::red());
    shape.set_outline_color(&Color::green());
    shape.set_outline_thickness(3.);
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        // Clear the window
        window.clear(&Color::black());
        window.draw(&shape);
        // Display things on screen
        window.display()

    }
}
