pub mod object {

    use gfx_graphics::Texture;
    use gfx_device_gl::Resources;

	pub struct Object {
		pub rot: f64,
		pub x: f64,
		pub y: f64,
		vetor_velocidade: [f64; 2],
		pub sprite: Option<Texture<Resources>>
	}

	impl Object {
		pub fn new() -> Object {
			Object {
				rot: 0.0,
				x: 0.0, y: 0.0,
				vetor_velocidade: [0.0, 0.0],
				sprite: None
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
	}
}
