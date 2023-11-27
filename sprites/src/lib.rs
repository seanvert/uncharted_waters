pub mod sprites {
	use model::model::Model;
    use piston_window::{Flip, PistonWindow, Texture, TextureContext, TextureSettings};

    pub fn on_load(window: &mut PistonWindow, model: &mut Model) {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
		// caminho dos assets
        let ships = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Ships")
            .unwrap();
		let ship_parts = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Ship parts")
            .unwrap();
		let _effects = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Effects")
            .unwrap();
		let tiles = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Tiles")
			.unwrap();

		// FIXME pegar isso de algum jeito melhor depois
		let mut current_object = model.objects.iter_mut();
		for num in 1..=96 {
			let string;
			if num < 10 {
				string = "0".to_string() + &num.to_string();
			} else {
				string = num.to_string();
			}
			let sprite = tiles.join("tile_".to_string() + &string + ".png");
			let sprite = Texture::from_path(
				&mut texture_context,
				&sprite,
				Flip::None,
				&TextureSettings::new(),
			).unwrap();
			
			current_object.next().unwrap().set_sprite(sprite);
		}

		// navio
        let ship_sprite = ships.join("ship (1).png");
        let ship_sprite = Texture::from_path(
            &mut texture_context,
            &ship_sprite,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        model.player.set_sprite(ship_sprite);

    // TODO pega as pastas
    //  TODO coloca os arquivos num vetor
    
    let cannon_ball_sprite = ship_parts.join("cannonBall.png");
    // TODO itera o vetor com as texturas
    let cannon_ball_sprite = Texture::from_path(
    		&mut texture_context,
    		&cannon_ball_sprite,
    		Flip::None,
    		&TextureSettings::new())
    		.unwrap();
	model.cannon_ball.set_sprite(cannon_ball_sprite);
	}
}
