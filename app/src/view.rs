pub mod view {
    use graphics::*;
    //	use piston_window::*;
    use crate::app::App;
    use crate::app::Object;
    use piston_window::RenderArgs; // TODO checar se é isto mesmo ou se é o outro
    use piston_window::{PistonWindow, clear, Image, Event, rectangle::square};
	
	pub struct Sprites {
	// TODO montar as sprites	
	}
	// TODO terminar esta função de carregar as sprites
	pub fn on_load (window : &PistonWindow) {
		use gfx_graphics::TextureContext;
		use gfx_graphics::Flip;
		use gfx_graphics::Texture;
		use gfx_graphics::TextureSettings;
		use piston::window::WindowSettings;
		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets").unwrap();
		let ships = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Ships").unwrap();
		let ship_parts = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Ship parts").unwrap();
		// TODO pega as pastas
		//  TODO coloca os arquivos num vetor
		let ship_sprite = ships.join("ship (1).png");
		let tank_sprite = assets.join("E-100_preview.png");
		let cannon_ball_sprite = ship_parts.join("cannonBall.png");
		let mut window: PistonWindow = WindowSettings::new("Uncharted Waters", [1366, 768])
			// .graphics_api(opengl)
			.exit_on_esc(true)
			.resizable(true)
			.fullscreen(true)
			.build()
			.unwrap();
			let mut texture_context = TextureContext {
				factory: window.factory.clone(),
				encoder: window.factory.create_command_buffer().into()
			};
			// TODO itera o vetor com as texturas
			let cannon_ball_sprite = Texture::from_path(
				&mut texture_context,
				&cannon_ball_sprite,
				Flip::None,
				&TextureSettings::new())
				.unwrap();
			let ship_sprite = Texture::from_path(
				&mut texture_context,
				&ship_sprite,
				Flip::None,
				&TextureSettings::new())
				.unwrap();
			let mut obje = Object::new();
			obje.set_sprite(ship_sprite);
			let obj = obje;
	}
    
    pub fn render (app: &mut App, args: &RenderArgs,
				   gl: &mut PistonWindow, e: &mut Event,
				   obj: Object
    ) {
		let image = Image::new().rect(square(0.0, 0.0, 1000.0));
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		let square = rectangle::square(0.0, 0.0, 100.0);
		let red = [1.0, 0.0, 0.0, 1.0];
		// self.window_size = [args.window_size[0], args.window_size[1]];
		gl.draw_2d(e, |c, g, _| {
			clear(GREEN, g);
			let transform = c.transform;
			// TODO fazer um loop nos objetos e renderizar as coordenadas e as sprites
			if let Some(sprite) = obj.sprite {
				let (sprite_x, sprite_y) = sprite.get_size();
				let (ocx, ocy) = (sprite_x / 2 , sprite_y / 2);
				let size = 0.1;
				image.draw(&sprite, &DrawState::new_alpha(),
						   transform.trans(app.player.x,
										   app.player.y)
						   .rot_rad(app.player.rot)
						   .trans(-(ocx as f64), -(ocy as f64))
						   .scale(size, size),
						   g);								
			} else {
				// fallback se não encontrar a sprite
				rectangle(red, square, transform.trans(app.player.x, app.player.y).rot_rad(app.player.rot).trans(-50.0, -50.0), g);
			};
		});
    }
    pub struct teste {
		pub t: f64,
    }
}
