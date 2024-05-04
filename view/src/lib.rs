pub mod view {
    // use graphics::*;
    use piston_window::ImageSize;
    use piston_window::RenderArgs;
    use piston_window::{
        math::Matrix2d, rectangle, rectangle::square, DrawState, Event, Graphics, Image,
        PistonWindow, Transformed, Window,
    };
    // use piston::window::WindowSettings;
    use model::model;
    use model::object;
    pub fn draw_object<G>(
        image: &Image,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        image.draw(texture, draw_state, transform, g);
    }

    pub fn render(
        model: &mut model::Model,
        _args: &RenderArgs,
        gl: &mut PistonWindow,
        e: &mut Event,
    ) {
        let image = Image::new().rect(square(0.0, 0.0, 1000.0));
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        // TODO: depois preciso trocar isso para o tamanho do viewport
        let size = gl.window.size();
        let (x, y) = (size.width, size.height);
        println!("res: {} x {}", size.width, size.height);
        let square = rectangle::square(0.0, 0.0, 100.0);
        gl.draw_2d(e, |c, g, _| {
            // TODO: dividir o background em tilesets
            // vou precisar do:
            // tamanho do viewport
            // tamanho do tileset
            // TODO: checar se o objeto está visível antes
            // object.visible

            // sea background
            let sea_sprite = &model.tile_textures["tile_73.png"];
            let (sea_sprite_x, sea_sprite_y) = sea_sprite.get_size();
			let tile_size = sea_sprite_x;
            let (ocx, ocy) = (sea_sprite_x / 2, sea_sprite_y / 2);
            let background_lines = x as u32 / tile_size;
            let background_columns = y as u32 / tile_size;
            let transform = c
                .transform;
            let x: u32 = sea_sprite_x;
            for lines in 0..=background_lines {
                for columns in 0..=background_columns {
                    /* draw_object(&image,
                    sea_sprite,
                    &DrawState::new_alpha(),
                    transform.trans((lines * x) as f64, (columns * x) as f64),
                    g); */

                    image.draw(
                        sea_sprite,
                        &DrawState::new_alpha(),
                        transform
                            .trans((lines * x) as f64, (columns * x) as f64)
                            .rot_rad(0.0)
                            .trans(-(ocx as f64), -(ocy as f64))
                            .scale(0.1, 0.1),
                        g,
                    );
                }
            }

            // objects render loop
            // for (_i, obj) in model.objects.iter().enumerate() {
            //     if let Some(sprite) = &obj.sprite {
            //         let (sprite_x, sprite_y) = sprite.get_size();
            //         let (ocx, ocy) = (sprite_x / 2, sprite_y / 2);
            //         let size = 0.1;
            //         image.draw(
            //             sprite,
            //             &DrawState::new_alpha(),
            //             transform
            //                 .trans(obj.x + x, obj.y + x)
            //                 .rot_rad(obj.rot)
            //                 .trans(-(ocx as f64), -(ocy as f64))
            //                 .scale(size, size),
            //             g,
            //         );
            //     }
            // 	x += 100.0;
            // }
            // player render
            if let Some(sprite) = &model.player.sprite {
                let (sprite_x, sprite_y) = sprite.get_size();
                let (ocx, ocy) = (sprite_x / 2, sprite_y / 2);
                let size = 0.1;
                image.draw(
                    sprite,
                    &DrawState::new_alpha(),
                    c.transform
                        .trans(model.player.x, model.player.y)
                        .rot_rad(model.player.rot)
                        .trans(-(ocx as f64), -(ocy as f64))
                        .scale(size, size),
                    g,
                );
            } else {
                // fallback se não encontrar a sprite
                rectangle(
                    RED,
                    square,
                    c.transform
                        .trans(model.player.x, model.player.y)
                        .rot_rad(model.player.rot)
                        .trans(-50.0, -50.0),
                    g,
                );
            };

            // cannon ball sprite
            if model.cannon_ball.visible {
                if let Some(sprite) = &model.cannon_ball.sprite {
                    let (sprite_x, sprite_y) = sprite.get_size();
                    let (ocx, ocy) = (sprite_x / 2, sprite_y / 2);
                    let size = 0.1;
                    image.draw(
                        sprite,
                        &DrawState::new_alpha(),
                        c.transform
                            .trans(model.cannon_ball.x, model.cannon_ball.y)
                            .rot_rad(model.cannon_ball.rot)
                            .trans(-(ocx as f64), -(ocy as f64))
                            .scale(size, size),
                        g,
                    );
                } else {
                    // fallback se não encontrar a sprite
                    rectangle(
                        RED,
                        square,
                        c.transform
                            .trans(model.cannon_ball.x, model.cannon_ball.y)
                            .rot_rad(model.cannon_ball.rot)
                            .trans(-50.0, -50.0),
                        g,
                    );
                };
            }
        });
    }
}
