#![windows_subsystem = "windows"]

extern crate piston;
extern crate piston_window;
extern crate image;
use piston_window::*;
use std::path::Path;

static COLOURS : [[u8;4]; 16] = [[255, 255, 255, 255], [0, 255, 255, 255], [255, 0, 255, 255], [255, 255, 0, 255],
					[192, 192, 192, 255], [255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255],
					[128, 128, 128, 255], [0, 128, 128, 255], [128, 0, 128, 255], [128, 128, 0, 255],
					[0, 0, 0, 255], [128, 0, 0, 255], [0, 128, 0, 255], [0, 0, 128, 255]];
static WIDTH : u32 = 500;
static HEIGHT : u32 = 500;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Voronoi", [WIDTH, HEIGHT])
		.exit_on_esc(true)
		.resizable(false)
		.build()
		.unwrap();
		
	let mut voronoi = Voronoi::new();
	voronoi.open_window(&mut window);
}

struct Voronoi {
	points : Vec<(u32, u32)>,
	distance_fn : u8,
	dotted : bool,
}

impl Voronoi {
	pub fn new() -> Self {
		Self { points: Vec::new(), distance_fn: 0, dotted: true }
	}

	pub fn open_window(&mut self, window: &mut PistonWindow) -> () {
		let mut cursor = [0.0, 0.0];
		while let Some(e) = window.next() {
			
			if let Some(_) = e.render_args() {
				self.draw_screen(window, &e);
			}

			if let Some(Button::Mouse(_button)) = e.press_args() {
				self.add_point((cursor[0] as u32, cursor[1] as u32));
			}
			
			if let Some(Button::Keyboard(key)) = e.press_args() {
				match key {
					Key::R => self.reset(),
					Key::S => self.save(),
					Key::D => self.change_dotted(),
					Key::Left => self.distance_fn = if self.distance_fn==0 { 2 } else { self.distance_fn - 1 },
					Key::Right => self.distance_fn = (self.distance_fn + 1) % 3,
				
					_ => (),
				}
			}
			
			e.mouse_cursor(|pos| {
				cursor = pos;
			});
		}
	}
	
	pub fn change_dotted(&mut self) {
		self.dotted = !self.dotted;
	}
	
	pub fn add_point(&mut self, point : (u32, u32)) {
		self.points.push(point);
	}
	
	pub fn reset(&mut self) {
		self.points = Vec::new();
		self.distance_fn = 0;
	}
	
	pub fn save(&mut self) {
		let mut path_string = "Voronoi-1.png".to_string();
		let mut path = Path::new(&path_string);
		let mut i = 2;
		while path.exists() {
			path_string = format!("Voronoi-{}.png", i);
			path = Path::new(&path_string);
			i += 1;
		}
		
		self.get_screen().save(path);
	}
	
	pub fn get_screen(&self) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
		let mut buffer_image = image::ImageBuffer::new(WIDTH, HEIGHT);
				
		for i in 0..(WIDTH*HEIGHT) {
			let y: u32 = (i / WIDTH) as u32;
			let x: u32 = (i % WIDTH) as u32;
			
			let closest = self.closest_point((x, y));
			
			buffer_image.put_pixel(x, y, image::Rgba(COLOURS[closest]));
		}
		
		return buffer_image;
	}
	
	pub fn closest_point(&self, (x, y): (u32, u32)) -> usize {
		let mut closest = 0;
		let mut distance = f64::MAX;
		
		for ((w, z), i) in self.points.iter().zip(0..(self.points.len())) {
			let new_distance = self.distance(x, y, *w, *z);
			if new_distance < distance {
				closest = i;
				distance = new_distance;
			}
		}
		if self.dotted {
			if distance < 3.0 { 12 } else {closest % COLOURS.len()}
		} else {
			closest % COLOURS.len()
		}
	}
	
	pub fn distance(&self, x: u32, y: u32, w: u32, z: u32) -> f64 {
		match self.distance_fn {
			0 => (((x as i32 - w as i32).pow(2) + (y as i32 - z as i32).pow(2)) as f64).sqrt(),
			1 => ((x as i32 - w as i32).abs() + (y as i32 - z as i32).abs()) as f64,
			2 => ((x as i32 - w as i32).abs().max((y as i32 - z as i32).abs())) as f64,
			
			_ => 1.0,
		}
	}
	
	pub fn draw_screen(&self, window: &mut PistonWindow, event: &Event) {
		let buffer_image = self.get_screen();
		
		let texture = Texture::from_image(
			&mut window.create_texture_context(),
			&buffer_image,
			&TextureSettings::new(),
		).unwrap();
		
		
		window.draw_2d(event, |_c, g, _| {
			image(&texture, _c.transform, g);
		});
	}
}