pub mod sprites {
    use model::model::Model;
    use piston_window::{Flip, PistonWindow, Texture, TextureContext, TextureSettings};
	use std::path::{PathBuf};

    pub fn on_load(window: &mut PistonWindow, model: &mut Model) {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let ships_path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Ships")
            .unwrap();
        let ship_parts_path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Ship parts")
            .unwrap();
        let tiles_path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Tiles")
            .unwrap();
		let effects_path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("Effects")
            .unwrap();
		// FIXME
		// read_folder(&mut texture_context, model, effects_path);
        // Load tile textures
        for entry in tiles_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "png" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let texture = load_texture(&mut texture_context, &path).unwrap();
                model.add_tile_texture(filename.to_string(), texture);
            }
        }
		for entry in effects_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "png" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let texture = load_texture(&mut texture_context, &path).unwrap();
                model.add_effects_texture(filename.to_string(), texture);
            }
		}
		for entry in ships_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "png" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let texture = load_texture(&mut texture_context, &path).unwrap();
                model.add_ships_texture(filename.to_string(), texture);
            }
		}
		for entry in ship_parts_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "png" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let texture = load_texture(&mut texture_context, &path).unwrap();
                model.add_ships_parts_texture(filename.to_string(), texture);
            }
        }

        // Load ship and cannon ball textures
		let ship_texture = model.ships_textures["ship (1).png"].clone();
		model.player.set_sprite(ship_texture);
		let cannon_ball_texture = model.ship_parts_textures["cannonBall.png"].clone();
        model.cannon_ball.set_sprite(cannon_ball_texture);
    }

	fn read_folder<F, R, C> (mut texture_context: TextureContext<F, R, C>, model: &mut Model, folder: PathBuf) -> Result<i64, String>
	where
		F: gfx_core::factory::Factory<R>,
		R: gfx_core::Resources,
		C: gfx_core::command::Buffer<R>,
	{
		for entry in folder.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "png" {
                // let filename = path.file_name().unwrap().to_str().unwrap();
                //let texture = load_texture(&mut texture_context, &path).unwrap();
				// FIXME
				// let texture = Texture::from_path(&mut texture_context, path, Flip::None, &TextureSettings::new()).unwrap();
                // model.add_tile_texture(filename.to_string(), texture);
            }
        }
		Ok(0)
	}

    fn load_texture<F, R, C>(texture_context: &mut TextureContext<F, R,C>, path: &PathBuf) -> Result<Texture<R>, String>
	where
		F: gfx_core::factory::Factory<R>,
		R: gfx_core::Resources,
		C: gfx_core::command::Buffer<R>,
	{
        Texture::from_path(texture_context, path, Flip::None, &TextureSettings::new())
    }
}
