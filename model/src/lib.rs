pub mod model {
    use controller::control;
    pub use object::object;
    use piston_window::UpdateArgs;
    use std::f64::consts::PI;
    // use crate::app::App;
    // use crate::object::object::Object;

    pub struct Model {
        pub player: object::Object,
        pub objects: Vec<object::Object>,
        pub controls: control::Control,
        pub window_size: [f64; 2],
    }

    impl Model {
        pub fn new() -> Model {
            Model {
                player: object::Object::new(),
                objects: object::Object::new_vec(100),
                controls: control::Control::new(),
                window_size: [0.0, 0.0],
            }
        }
    }

    pub fn update(model: &mut Model, args: &UpdateArgs) {
        let speed = 500.0;

        if model.controls.up_d {
            if model.player.y > 0.0 {
                model.player.mov(0.0, -speed * args.dt);
                if model.controls.right_d {
                    model.player.rot_to(PI / 4.0);
                } else if model.controls.left_d {
                    model.player.rot_to(3.0 * PI / 4.0);
                } else {
                    model.player.rot_to(PI);
                }
            }
        }

        if model.controls.down_d {
            if model.player.y < model.window_size[1] - 50.0 {
                model.player.mov(0.0, speed * args.dt);
                if model.controls.right_d {
                    model.player.rot_to(-PI / 4.0);
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
                    model.player.rot_to(5.0 * PI / 4.0);
                } else {
                    model.player.rot_to(PI / 2.0);
                }
            }
        }

        if model.controls.right_d {
            if model.player.x < model.window_size[0] - 50.0 {
                model.player.mov(speed * args.dt, 0.0);
                if model.controls.up_d {
                    model.player.rot_to(PI / 4.0);
                } else if model.controls.down_d {
                    model.player.rot_to(-PI / 4.0);
                } else {
                    model.player.rot_to(3.0 * PI / 2.0);
                }
            }
        }
    }
}
