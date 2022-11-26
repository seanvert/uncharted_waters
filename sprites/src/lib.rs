pub mod sprites {
	use piston_window::{PistonWindow, TextureContext,
						Texture, TextureSettings, Flip};
	use object::object::Object;
	
	pub fn on_load (window: &mut PistonWindow, player: &mut Object) {
		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets").unwrap();
		let mut texture_context = TextureContext {
			factory: window.factory.clone(),
			encoder: window.factory.create_command_buffer().into()
		};
		let ships = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Ships").unwrap();
		let ship_sprite = ships.join("ship (1).png");
		let ship_sprite = Texture::from_path(
			&mut texture_context,
			&ship_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		player.set_sprite(ship_sprite);
	}

	// 	let ship_parts = find_folder::Search::ParentsThenKids(3, 3)
	// 		.for_folder("Ship parts").unwrap();
	// 	// TODO pega as pastas
	// 	//  TODO coloca os arquivos num vetor
	// 	let ship_sprite = ships.join("ship (1).png");
	// 	let tank_sprite = assets.join("E-100_preview.png");
	// 	let cannon_ball_sprite = ship_parts.join("cannonBall.png");
	// 	// TODO itera o vetor com as texturas
	// 	let cannon_ball_sprite = Texture::from_path(
	// 		&mut texture_context,
	// 		&cannon_ball_sprite,
	// 		Flip::None,
	// 		&TextureSettings::new())
	// 		.unwrap();
	// }	
}
