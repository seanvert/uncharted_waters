pub mod object {

    use gfx_device_gl::Resources;
    use gfx_graphics::Texture;

	#[derive(Debug)]
    pub struct Object {
        pub rot: f64,
        pub x: f64,
        pub y: f64,
        pub sprite: Option<Texture<Resources>>,
    }

    impl Object {
        pub fn new() -> Object {
            Object {
                rot: 0.0,
                x: 0.0,
                y: 0.0,
                sprite: None,
            }
        }

        pub fn new_vec(n_objects: usize) -> Vec<Object> {
            let mut objects: Vec<Object> = Vec::with_capacity(n_objects);
            for _i in 0..n_objects {
                objects.push(Object::new());
            }
            objects
        }
        pub fn get_rot(&mut self) -> f64 {
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
