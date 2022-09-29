// extern crate gfx_device_gl;
// extern crate gfx;
// extern crate find_folder;
// extern crate gfx_graphics;
// extern crate gfx;

// use piston_window::*;

// use gfx_graphics::GfxGraphics;
// use std::f64;
pub mod object {

    use gfx_graphics::Texture;
    use gfx_device_gl::Resources;
	// CommandBuffer};
	pub struct Object {

		pub rot: f64,
		pub x: f64,
		pub y: f64,
		pub no_ar: bool,
		shoot: [f64; 2],
		cannon: [f64; 2],
		vetor_velocidade: [f64; 2],
		// cannon_ball_sprite: Option<Texture<Resources>>,
		pub sprite: Option<Texture<Resources>>
	}

	impl Object {
		pub fn new() -> Object {
			Object {rot: 0.0,
					x: 0.0, y: 0.0,
					no_ar: false,
					shoot: [0.0, 0.0],
					cannon: [0.0, 0.0],
					vetor_velocidade: [0.0, 0.0],
				// cannon_ball_sprite: None,
				sprite: None
			}
		}

		pub fn shoot_cannon(&mut self, coordenadas: [f64; 2]) {
			if !self.no_ar {
				self.no_ar = true;
				self.shoot = coordenadas;
				self.cannon = [self.x, self.y];
				let [x, y] = self.shoot;
				let [a, b] = self.cannon;
				if x - a > 0.0 {
					self.vetor_velocidade[0] = 1.0;
				} else {
					self.vetor_velocidade[0] = -1.0;
				}
				if y - b > 0.0 {
					self.vetor_velocidade[1] = 1.0;
				} else {
					self.vetor_velocidade[1] = -1.0;
				}
			}
		}

		pub fn move_cannon_ball(&mut self, velocidade: f64) {
			self.cannon[0] += self.vetor_velocidade[0] * velocidade;
			self.cannon[1] += self.vetor_velocidade[1] * velocidade;
			let [x, y] = self.shoot;
			let [a, b] = self.cannon;
			let [c, d] = [x - a, y - b];
			if (c.powi(2) + d.powi(2)).sqrt() < 300.0 {
				self.no_ar = false;
			}

		}

		pub fn get_rot(&mut self) -> f64{
			return self.rot;
		}
		
		pub fn rot(&mut self, r: f64) {
			self.rot += r;
		}

		pub fn rot_to(&mut self, r: f64) {
			self.rot = r;
		}
		
		pub fn mov(&mut self, x: f64, y: f64) {
			self.x += x;
			self.y += y;
		}

		pub fn fwd(&mut self, d: f64) {
			self.x += d * (-self.rot.sin());
			self.y += d * self.rot.cos();
		}

		pub fn mov_to(&mut self, x: f64, y: f64) {
			self.x = x;
			self.y = y;
		}

		pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
			self.sprite = Some(sprite);
		}

		// pub fn set_cannon_ball_sprite(&mut self, sprite: Texture<Resources>) {
		// 	self.cannon_ball_sprite = Some(sprite);
		// }
		
		// pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
		// 	let square = rectangle::square(0.0, 0.0, 100.0);
		// 	let red = [1.0, 0.0, 0.0, 1.0];
		
		
		// 	match self.sprite {
		// 		None => {
		// 			rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
		// 		}
		// 		Some(ref sprite) => {
		// 			let (sprite_x, sprite_y) = sprite.get_size();
		// 			let (ocx, ocy) = (sprite_x / 2, sprite_y / 2);
		// 			image(sprite, view.trans(self.x, self.y)
		// 				  .rot_rad(self.rot)
		// 				  .trans(-(ocx as f64), -(ocy as f64)), g);
		// 		}
		// 	}
		// 	if self.no_ar {
		// 		match self.cannon_ball_sprite {
		// 			None => {
		// 				rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
		// 			}
		// 			Some(ref sprite) => {
		// 				let (sprite_x, sprite_y) = sprite.get_size();
		// 				let (ocx, ocy) = (sprite_x / 2, sprite_y / 2);
		// 				let [x, y] = self.cannon;
		// 				image(sprite, view.trans(x, y)
		// 					  .rot_rad(self.rot)
		// 					  .trans(-(ocx as f64), -(ocy as f64)), g);
		// 			}
		// 		}
		// 	}
		
		// }
	}
}
