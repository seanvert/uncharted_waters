pub mod control {
	use piston::input::{Input, Key, Button, MouseButton};
	use piston::ButtonState;
	#[derive(Debug)]
	pub struct Control {
		pub up_d: bool,
		pub down_d: bool,
		pub left_d: bool,
		pub right_d: bool,
		pub cursor: [f64; 2],
	}

	impl Control {
		pub fn new () -> Control {
			Control {
				up_d: false,
				down_d: false,
				left_d: false,
				right_d: false,
				cursor: [0.0, 0.0],
			}
		}

		pub fn change_controls (&mut self) {
			// TODO adicionar as coisas para mudar os controles
		}
		pub fn mouse_input(&mut self, coordinates: [f64; 2]) {
			self.cursor = coordinates;
		}
		pub fn on_input(&mut self, inp: Input) {
			match inp {
				Input::Button(but) => {
					match but.state {
						ButtonState::Press => {
							// DEBUG printa os controles
							// println!("{:#?}", but);
							match but.button {
								Button::Keyboard(Key::Up) => {
									self.up_d = true;
								}
								Button::Keyboard(Key::Down) => {
									self.down_d = true;
								}
								Button::Keyboard(Key::Left) => {
									self.left_d = true;
								}
								Button::Keyboard(Key::Right) => {
									self.right_d = true;
								}
								Button::Mouse(MouseButton::Left) => {
									// DEBUG
									println!("{:#?}", self.cursor);
								}
								_ => {}
							}
						}
						ButtonState::Release => {
							match but.button {
								Button::Keyboard(Key::Up) => {
									self.up_d = false;
								}
								Button::Keyboard(Key::Down) => {
									self.down_d = false;
								}
								Button::Keyboard(Key::Left) => {
									self.left_d = false;
								}
								Button::Keyboard(Key::Right) => {
									self.right_d = false;
								}
								_ => {}
							}
						}
					}				
				}
				_ => {}
 			}
		}
	}
}
