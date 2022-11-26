pub mod object {
// TODO: acho que o certo seria encapsular a sprite num tipo próprio dela
    use gfx_graphics::Texture;
    use gfx_device_gl::Resources;

	pub struct Object {
		pub rot: f64,
		pub x: f64,
		pub y: f64,
		pub sprite: Option<Texture<Resources>>
	}

	impl Object {
		pub fn new() -> Object {
			Object {
				rot: 0.0,
				x: 0.0, y: 0.0,
				sprite: None
			}
		}
		// TODO: acho que deveria trocar isso para um tipo cena
		pub fn new_vec() -> Vec<Object> {
			let mut objects: Vec<Object> = Vec::with_capacity(30);
			for _i in 0..29 {
				objects.push(Object::new());
			}
			objects
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
