pub mod model {
	use std::f64::consts::PI;
	use piston_window::UpdateArgs;
	use crate::app::App;
	use crate::object::object::Object;

	pub fn update(app: &mut App, args: &UpdateArgs) {
		let speed = 500.0;

		if app.controls.up_d {
			if app.player.y > 0.0 {
				app.player.mov(0.0, -speed * args.dt);
				if app.controls.right_d {
					app.player.rot_to(PI/4.0);
				} else if app.controls.left_d {
					app.player.rot_to(3.0*PI/4.0);
				} else {
					app.player.rot_to(PI);
				}
			}
		}
		
		if app.controls.down_d {
			if app.player.y < app.window_size[1] - 50.0 {
				app.player.mov(0.0, speed * args.dt);
				if app.controls.right_d {
					app.player.rot_to(-PI/4.0);
				} else if app.controls.left_d {
					app.player.rot_to(5.0*PI/4.0);
				} else {
					app.player.rot_to(0.0);
				}
			}
		}
		
		if app.controls.left_d {
			if app.player.x > 0.0 {
				app.player.mov(-speed * args.dt, 0.0);
				if app.controls.up_d {
					app.player.rot_to(3.0*PI/4.0);
				} else if app.controls.down_d {
					app.player.rot_to(5.0*PI/4.0);
				} else {
					app.player.rot_to(PI/2.0);
				}
			}
		}
		
		if app.controls.right_d {
			if app.player.x < app.window_size[0] - 50.0 {
				app.player.mov(speed * args.dt, 0.0);
				if app.controls.up_d {
					app.player.rot_to(PI/4.0);
				} else if app.controls.down_d {
					app.player.rot_to(-PI/4.0);
				} else {
					app.player.rot_to(3.0*PI/2.0);
				}
			}
		}
	}
}
