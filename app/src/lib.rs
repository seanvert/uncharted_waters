extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx;

mod object;
mod view;
mod controller;
mod model;

pub mod app {
    pub use crate::object::object::Object;
    pub use crate::view::view::render;
    pub use crate::controller::control;
    pub use crate::model::model::update;
    use gfx_device_gl::{Resources, CommandBuffer};
    use piston::event_loop::{EventSettings, Events};
    use piston::input::Input::Move;
    use piston::ButtonState;
    use piston::window::WindowSettings;
    use piston_window::*;
    use graphics::*;
    use graphics::rectangle::square;

    pub struct App {
		pub controls: control::Control,
		// TODO trocar isso para uma lista de objetos
		pub player: Object,
		pub window_size: [f64; 2],
    }

    impl App {
		pub fn new() -> App {
			App {
				player: Object::new(),
				controls: control::Control::new(),
				window_size: [0.0, 0.0],
			}
		}

		pub fn run(&mut self) {
			let opengl = OpenGL::V3_2;
			let mut window: PistonWindow = WindowSettings::new("Uncharted Waters", [1366, 768])
				.graphics_api(opengl)
				.exit_on_esc(true)
				.resizable(true)
				.fullscreen(true)
				.build()
				.unwrap();
			window.set_max_fps(30);
			let mut events = Events::new(EventSettings::new());
			println!("{} {}", window.size().width, window.size().height);
			// TODO colocar uma rotina para criar um menu
			// TODO opção para carregar o jogo

			while let Some(mut e) = events.next(&mut window) {

				if let Some(args) = e.render_args() {
					// TODO ajeitar o path disso
					// FIXME passar um iterável com a lista de objetos
					// atualmente está passando só um objeto
					// app vai precisar conter um iterável com os
					// objetos que serão renderizados na cena
					// FIXME colocar o obj no model depois e passar
					// pra cá
					let mut obj = Object::new();
					crate::view::view::render(self, &args,
											  &mut window, &mut e,
											  obj);
				}
				if let Some(args) = e.update_args() {
					crate::model::model::update(self, &args);
				}
				if let Some(inp) = e.button_args() {
					self.controls.on_input(piston::Input::Button(inp));
				}
				if let Some(coordinates) = e.mouse_cursor_args() {
					self.controls.mouse_input(coordinates);
				}
				if let Some(focus) = e.focus_args() {
					println!("{:?}", focus);
				}
				if let Some(args) = e.resize_args() {
					self.window_size = args.window_size;
					println!("{:?}", args);
				}
			}
		}
    }
}
