extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::*;

pub struct App {
	gl: GlGraphics,
	x: f64,
	y: f64,
	size_x: f64,
	size_y: f64,
}

impl App {
	pub fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const GREEN: [f32; 4] = [0.10, 50.0, 1.0, 5.0];
		const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let x_pos = self.x as f64;
		let y_pos = self.y as f64;
		let x_size = self.size_x as f64;
		let y_size = self.size_y as f64;

		let square = rectangle::square(0.0, 0.0, 50.0);

		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREEN, gl);

			let transform = c.transform.trans(x_pos, y_pos).trans(0.1, 0.1);
			rectangle(RED, [0.0, 0.0, x_size, y_size], transform, gl);
		});
	}

	fn press(&mut self, args: &Button) {
		const d: i64 = 20;

		if let &Button::Keyboard(key) = args {
			match key {
				Key::Up => {
					let mut num_i64 = self.y as i64;
					num_i64 -= d;
					self.y = num_i64 as f64;

					if self.y < -20.0 as f64 {self.y = 580 as f64;}
				}
				Key::Down => {
					let mut num_i64 = self.y as i64;
					num_i64 += d;
					self.y = num_i64 as f64;

					if self.y > 580 as f64 {self.y = -20.0 as f64;}
				}
				Key::Left => {
					let mut num_i64 = self.x as i64;
					num_i64 -= d;
					self.x = num_i64 as f64;

					if self.x < -20.0 as f64 {self.x = 580 as f64;}
				}
				Key::Right => {
					let mut num_i64 = self.x as i64;
					num_i64 += d;
					self.x = num_i64 as f64;

					if self.x > 580 as f64 {self.x = -20.0 as f64;}
				}
				_ => {}
			}
		}
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new("RUST", [600, 600])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut app = App {
		gl: GlGraphics::new(opengl),
		x: 0.0,
		y: 0.0,
		size_x: 20.0,
		size_y: 20.0,
	};

	let mut events = Events::new(EventSettings::new().ups(8));
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {app.render(&args);}
		if let Some(b) = e.press_args() {app.press(&b);}
	}
}

