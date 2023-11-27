pub mod model {
    use controller::control;
    pub use object::object;
    use piston_window::UpdateArgs;
    use std::f64::consts::PI;
    // use crate::app::App;
    // use crate::object::object::Object;

    pub struct Model {
        pub player: object::Object,
		pub cannon_ball: object::Object,
		pub target_coords: [f64; 2],
        pub objects: Vec<object::Object>,
        pub controls: control::Control,
        pub window_size: [f64; 2],
    }

    impl Model {
        pub fn new() -> Model {
            Model {
                player: object::Object::new(true),
				cannon_ball: object::Object::new(false),
				target_coords: [0.0, 0.0],
                objects: object::Object::new_vec(100),
                controls: control::Control::new(),
                window_size: [0.0, 0.0],
            }
        }
    }

	pub fn update_cannon_ball (model: &mut Model, args: &UpdateArgs) {
		let max_cannon_ball_distance = 100.0;
		let speed = 20.0;
		if model.cannon_ball.visible {
			// não atira dependendo coloco um som
			// move o modelo da bola
			// faz ela sumir se chegou no 'limite'
			let [x, y] = model.target_coords;
			model.cannon_ball.fwd(speed);
		} else {
			// se não tem uma bola de canhão no ar
			// atira
			if model.controls.left_click {
				model.cannon_ball.mov_to(model.player.x, model.player.y);
				model.cannon_ball.set_visible();
				model.target_coords = model.controls.cursor;
				model.cannon_ball.turn_to_coordinate(model.player.x, model.player.y);
			}
		}
	}

    pub fn update_player(model: &mut Model, args: &UpdateArgs) {
        let speed = 500.0;

        if model.controls.up_d {
            if model.player.y > 0.0 {
                model.player.mov(0.0, -speed * args.dt);
                if model.controls.right_d {
                    model.player.rot_to(PI / 4.0);
                } else if model.controls.left_d {
                    model.player.rot_to(7.0 * PI / 4.0);
                } else {
                    model.player.rot_to(PI);
                }
            }
        }

        if model.controls.down_d {
            if model.player.y < model.window_size[1] - 50.0 {
                model.player.mov(0.0, speed * args.dt);
                if model.controls.right_d {
                    model.player.rot_to(3.0 * PI / 4.0);
                } else if model.controls.left_d {
                    model.player.rot_to(5.0 * PI / 4.0);
                } else {
                    model.player.rot_to(0.0);
                }
            }
        }

        if model.controls.left_d {
            if model.player.x > 0.0 {
                model.player.mov(-speed * args.dt, 0.0);
                if model.controls.up_d {
                    model.player.rot_to(3.0 * PI / 4.0);
                } else if model.controls.down_d {
                    model.player.rot_to(PI / 4.0);
                } else {
                    model.player.rot_to(PI / 2.0);
                }
            }
        }

        if model.controls.right_d {
            if model.player.x < model.window_size[0] - 50.0 {
                model.player.mov(speed * args.dt, 0.0);
                if model.controls.up_d {
                    model.player.rot_to(5.0 * PI / 4.0);
                } else if model.controls.down_d {
                    model.player.rot_to(-PI / 4.0);
                } else {
                    model.player.rot_to(3.0 * PI / 2.0);
                }
            }
        }
    }
}
