pub mod view {
    // use graphics::*;
    use piston_window::RenderArgs;
    use piston_window::{PistonWindow, clear, Image,
						Event, rectangle, Transformed,
						rectangle::square, DrawState,
	Window};
	use piston_window::ImageSize;
	// use piston::window::WindowSettings;
	use model::model;
    
    pub fn render (app: &mut model::Model, args: &RenderArgs,
				   gl: &mut PistonWindow, e: &mut Event
    ) {
		let image = Image::new().rect(square(0.0, 0.0, 1000.0));
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
		const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
		let (x, y) = (0.0, 0.0);
		// TODO: depois preciso trocar isso para o tamanho do viewport
		let size = gl.window.size();
		// let (x, y) = (size.width, size.height);
		let square = rectangle::square(0.0, 0.0, 100.0);
		gl.draw_2d(e, |c, g, _| {
			// TODO: dividir o background em tilesets
			// vou precisar do:
			// tamanho do viewport
			// tamanho do tileset

			let transform = c.transform;
			for i in 0..10 {
				for j in 0..10 {
					let n = i as f64;
					let m = j as f64;
					let offset_x = n * 30.0;
					let offset_y = m * 30.0;
					rectangle(RED, rectangle::square(offset_x, offset_y, 30.0), transform.trans(x, y), g);
					rectangle(GREEN, rectangle::square(30.0, 0.0, 30.0), transform.trans(x, y), g);
					rectangle(BLUE, rectangle::square(0.0, 30.0, 30.0), transform.trans(x, y), g);
					rectangle(YELLOW, rectangle::square(30.0, 30.0, 30.0), transform.trans(x, y), g);
				}
			}
			// objects render loop
			for (i, obj) in app.objects.iter().enumerate() {
				if let Some(sprite) = &obj.sprite {
					let (sprite_x, sprite_y) = sprite.get_size();
					let (ocx, ocy) = (sprite_x / 2 , sprite_y / 2);
					let size = 0.1;
					image.draw(sprite, &DrawState::new_alpha(),
							   transform.trans(obj.x,
											   obj.y)
							   .rot_rad(obj.rot)
							   .trans(-(ocx as f64), -(ocy as f64))
							   .scale(size, size),
							   g);
				}
			}
			// player render
			if let Some(sprite) = &app.player.sprite {
				let (sprite_x, sprite_y) = sprite.get_size();
				let (ocx, ocy) = (sprite_x / 2 , sprite_y / 2);
				let size = 0.1;
				image.draw(sprite, &DrawState::new_alpha(),
						   transform.trans(app.player.x,
										   app.player.y)
						   .rot_rad(app.player.rot)
						   .trans(-(ocx as f64), -(ocy as f64))
						   .scale(size, size),
						   g);								
			} else {
				// fallback se não encontrar a sprite
				rectangle(RED, square,
						  transform.trans(app.player.x, app.player.y)
						  .rot_rad(app.player.rot)
						  .trans(-50.0, -50.0), g);
			};
		});
    }
}
