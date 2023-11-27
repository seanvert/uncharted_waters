// extern crate glutin_window;
// extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
// extern crate gfx_graphics;
// extern crate gfx_device_gl;
// extern crate find_folder;
// extern crate gfx;

pub mod app {
    pub use model::model;
    pub use sprites::sprites;
    pub use view::view::render;
    // use gfx_device_gl::{Resources, CommandBuffer};
    use piston::event_loop::{EventSettings, Events};
    //use piston::input::Input::Move;
    //use piston::ButtonState;
    use crate::piston::{
        ButtonEvent, EventLoop, FocusEvent, MouseCursorEvent, RenderEvent, ResizeEvent,
        UpdateEvent, Window,
    };
    use piston::window::WindowSettings;
    use piston_window::OpenGL;
    use piston_window::PistonWindow;
    // use graphics::*;
    // use graphics::rectangle::square;

    pub struct App {
        pub model: model::Model,
    }

    impl App {
        pub fn new() -> App {
            App {
                model: model::Model::new(),
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
            sprites::on_load(&mut window, &mut self.model);
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
                    view::view::render(&mut self.model, &args, &mut window, &mut e);
                }
                if let Some(args) = e.update_args() {
                    model::update_player(&mut self.model, &args);
					model::update_cannon_ball(&mut self.model, &args);
                }
                if let Some(inp) = e.button_args() {
                    self.model.controls.on_input(piston::Input::Button(inp));
                }
                if let Some(coordinates) = e.mouse_cursor_args() {
                    self.model.controls.mouse_input(coordinates);
                }
                if let Some(focus) = e.focus_args() {
                    println!("{:?}", focus);
                }
                if let Some(args) = e.resize_args() {
                    self.model.window_size = args.window_size;
                    println!("{:?}", args);
                }
            }
        }
    }
}

use crate::app::App;
fn main() {
    let mut jogo = crate::App::new();
    jogo.run();
}
